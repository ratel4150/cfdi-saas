// File: crates/cfdi-core/src/models/cfdi.rs
use serde::{Deserialize, Serialize};
use chrono::Local;
use uuid::Uuid;
use super::{emisor::Emisor, receptor::Receptor, concepto::Concepto};
use crate::models::carta_porte::CartaPorte;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfdi {
    pub version: String,
    pub serie: Option<String>,
    pub folio: Option<String>,
    pub fecha: String,              // ISO 8601: 2026-04-16T10:00:00
    pub forma_pago: FormaPago,
    pub condiciones_pago: Option<String>,
    pub sub_total: f64,
    pub descuento: Option<f64>,
    pub moneda: Moneda,
    pub tipo_cambio: Option<f64>,
    pub total: f64,
    pub tipo_comprobante: TipoComprobante,
    pub exportacion: String,        // "01" = No aplica
    pub metodo_pago: MetodoPago,
    pub lugar_expedicion: String,   // código postal
    pub emisor: Emisor,
    pub receptor: Receptor,
    pub conceptos: Vec<Concepto>,
    pub carta_porte: Option<CartaPorte>,
    // Campos que llena el PAC después del timbrado
    pub sello: Option<String>,
    pub no_certificado: Option<String>,
    pub certificado: Option<String>,
    pub uuid: Option<String>,
    pub sello_sat: Option<String>,
    pub no_certificado_sat: Option<String>,
    pub fecha_timbrado: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoComprobante {
    #[serde(rename = "I")] Ingreso,
    #[serde(rename = "E")] Egreso,
    #[serde(rename = "T")] Traslado,
    #[serde(rename = "P")] Pago,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormaPago {
    #[serde(rename = "01")] Efectivo,
    #[serde(rename = "03")] TransferenciaElectronica,
    #[serde(rename = "04")] TarjetaCredito,
    #[serde(rename = "28")] TarjetaDebito,
    #[serde(rename = "99")] PorDefinir,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetodoPago {
    #[serde(rename = "PUE")] PagoEnUnaSolaExhibicion,
    #[serde(rename = "PPD")] PagoParcialidadesDiferidos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Moneda {
    #[serde(rename = "MXN")] PesoMexicano,
    #[serde(rename = "USD")] DolarAmericano,
}

impl Cfdi {
    pub fn nuevo_ingreso(
        serie: Option<&str>,
        folio: Option<&str>,
        lugar_expedicion: &str,
        emisor: Emisor,
        receptor: Receptor,
        conceptos: Vec<Concepto>,
    ) -> Self {
        let sub_total: f64 = conceptos.iter().map(|c| c.importe).sum();
        let total_iva: f64 = conceptos.iter()
            .filter_map(|c| c.impuestos.as_ref())
            .flat_map(|imp| imp.traslados.iter())
            .map(|t| t.importe)
            .sum();
        let total = (( sub_total + total_iva) * 100.0).round() / 100.0;

        Self {
            version: "4.0".to_string(),
            serie: serie.map(|s| s.to_string()),
            folio: folio.map(|f| f.to_string()),
            fecha: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
            forma_pago: FormaPago::TransferenciaElectronica,
            condiciones_pago: None,
            sub_total,
            descuento: None,
            moneda: Moneda::PesoMexicano,
            tipo_cambio: None,
            total,
            tipo_comprobante: TipoComprobante::Ingreso,
            exportacion: "01".to_string(),
            metodo_pago: MetodoPago::PagoEnUnaSolaExhibicion,
            lugar_expedicion: lugar_expedicion.to_string(),
            emisor,
            receptor,
            conceptos,
            carta_porte: None,
            sello: None,
            no_certificado: None,
            certificado: None,
            uuid: None,
            sello_sat: None,
            no_certificado_sat: None,
            fecha_timbrado: None,
        }
    }
}