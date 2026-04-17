use sha2::{Sha256, Digest};
use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey, signature::{SignatureEncoding, Signer}};
use rsa::pkcs1v15::SigningKey;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::error::{CfdiError, CfdiResult};

/// Genera la cadena original del CFDI aplicando la transformación
/// que especifica el SAT en el Anexo 20.
/// Formato: ||campo1|campo2|campo3||
pub fn generar_cadena_original(xml: &str) -> CfdiResult<String> {
    // Extrae atributos del nodo raíz en el orden exacto del SAT
    let mut cadena = String::with_capacity(512);
    cadena.push_str("||");

    let campos = [
        "Version", "Serie", "Folio", "Fecha", "FormaPago",
        "NoCertificado", "SubTotal", "Descuento", "Moneda",
        "TipoCambio", "Total", "TipoDeComprobante", "Exportacion",
        "MetodoPago", "LugarExpedicion", "Confirmacion",
    ];

    for campo in &campos {
        if let Some(valor) = extraer_atributo(xml, campo) {
            cadena.push_str(&valor);
            cadena.push('|');
        }
    }

    // Emisor
    for campo in &["Rfc", "Nombre", "RegimenFiscal"] {
        if let Some(valor) = extraer_atributo_de_nodo(xml, "cfdi:Emisor", campo) {
            cadena.push_str(&valor);
            cadena.push('|');
        }
    }

    // Receptor
    for campo in &["Rfc", "Nombre", "DomicilioFiscalReceptor",
                   "ResidenciaFiscal", "NumRegIdTrib",
                   "RegimenFiscalReceptor", "UsoCFDI"] {
        if let Some(valor) = extraer_atributo_de_nodo(xml, "cfdi:Receptor", campo) {
            cadena.push_str(&valor);
            cadena.push('|');
        }
    }

    cadena.push('|');
    Ok(cadena)
}

/// Firma la cadena original con la llave privada CSD usando SHA-256 + RSA
/// y devuelve el sello en base64 como lo requiere el SAT.
pub fn firmar_cadena(cadena: &str, llave_pem: &str) -> CfdiResult<String> {
    // Carga la llave privada desde PEM
    let llave = RsaPrivateKey::from_pkcs8_pem(llave_pem)
        .map_err(|e| CfdiError::CryptoError(format!(
            "No se pudo cargar la llave privada CSD: {e}"
        )))?;

    // Crea el signing key con SHA-256
    let signing_key = SigningKey::<Sha256>::new(llave);

    // Firma la cadena original
    let firma = signing_key.sign(cadena.as_bytes());

    // Devuelve el sello en base64 estándar
    Ok(BASE64.encode(firma.to_bytes()))
}

/// Calcula el SHA-256 de la cadena original (para verificación)
pub fn hash_sha256(texto: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(texto.as_bytes());
    format!("{:x}", hasher.finalize())
}

// ── Helpers de parsing XML simple ────────────────────────

fn extraer_atributo(xml: &str, atributo: &str) -> Option<String> {
    let patron = format!(r#"{}=""#, atributo);
    let inicio = xml.find(&patron)? + patron.len();
    let fin = xml[inicio..].find('"')? + inicio;
    let valor = &xml[inicio..fin];
    if valor.is_empty() { None } else { Some(valor.to_string()) }
}

fn extraer_atributo_de_nodo(xml: &str, nodo: &str, atributo: &str) -> Option<String> {
    let inicio_nodo = xml.find(nodo)?;
    let fin_nodo = xml[inicio_nodo..].find("/>")
        .map(|i| i + inicio_nodo)
        .unwrap_or(xml.len());
    let fragmento = &xml[inicio_nodo..fin_nodo];
    extraer_atributo(fragmento, atributo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cadena_original_contiene_version() {
        let xml = r#"<cfdi:Comprobante Version="4.0" Serie="F" Folio="1"
            Fecha="2026-04-16T10:00:00" FormaPago="03"
            SubTotal="1000.00" Moneda="MXN" Total="1160.00"
            TipoDeComprobante="I" Exportacion="01"
            MetodoPago="PUE" LugarExpedicion="06600"
            Sello="" NoCertificado="" Certificado="">
            <cfdi:Emisor Rfc="XAXX010101000"
                Nombre="TEST" RegimenFiscal="601"/>
            <cfdi:Receptor Rfc="XAXX010101000"
                Nombre="RECEPTOR" DomicilioFiscalReceptor="06600"
                RegimenFiscalReceptor="601" UsoCFDI="G03"/>
        </cfdi:Comprobante>"#;

        let cadena = generar_cadena_original(xml).unwrap();

        assert!(cadena.starts_with("||"));
        assert!(cadena.contains("4.0"));
        assert!(cadena.contains("XAXX010101000"));
        assert!(cadena.contains("1000.00"));

        println!("Cadena original:\n{}", cadena);
    }

    #[test]
    fn test_hash_sha256() {
        let texto = "||4.0|F|1|2026-04-16T10:00:00||";
        let hash = hash_sha256(texto);
        assert_eq!(hash.len(), 64); // SHA-256 = 64 chars hex
        println!("SHA-256: {}", hash);
    }
}