// File: crates/api/src/handlers/cfdi.rs
use axum::{extract::{Extension, State}, Json};
use serde::{Deserialize, Serialize};
use cfdi_core::{
    Cfdi, Emisor, Receptor, Concepto, RegimenFiscal, UsoCfdi,
    xml::generador::generar_xml,
    crypto::sello::{generar_cadena_original, hash_sha256},
};
use crate::{error::ApiResult, middleware::auth::AuthUser, state::AppState};

// ── Request / Response types ──────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EmitirCfdiRequest {
    pub serie:             Option<String>,
    pub folio:             Option<String>,
    pub lugar_expedicion:  String,
    pub receptor_rfc:      String,
    pub receptor_nombre:   String,
    pub receptor_cp:       String,
    pub receptor_regimen:  String,
    pub conceptos:         Vec<ConceptoRequest>,
    pub con_carta_porte:   bool,
}

#[derive(Debug, Deserialize)]
pub struct ConceptoRequest {
    pub clave_prod_serv: String,
    pub clave_unidad:    String,
    pub cantidad:        f64,
    pub descripcion:     String,
    pub valor_unitario:  f64,
}

#[derive(Debug, Serialize)]
pub struct EmitirCfdiResponse {
    pub xml:              String,
    pub cadena_original:  String,
    pub hash_sha256:      String,
    pub sub_total:        f64,
    pub total:            f64,
    pub estado:           String,
}

// ── Handlers ──────────────────────────────────────────────

/// POST /api/v1/cfdi/emitir
/// Genera el XML del CFDI con cadena original y hash
pub async fn emitir_cfdi(
    State(_state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<EmitirCfdiRequest>,
) -> ApiResult<Json<EmitirCfdiResponse>> {

    // Construye el emisor desde el RFC del token JWT
    let emisor = Emisor::new(
        &user.rfc,
        &user.org,
        RegimenFiscal::GeneralLeyPersonasMorales,
    )?;

    // Construye el receptor
    let receptor = Receptor::new(
        &req.receptor_rfc,
        &req.receptor_nombre,
        &req.receptor_cp,
        &req.receptor_regimen,
        UsoCfdi::GastosEnGeneral,
    )?;

    // Construye los conceptos
    let conceptos: Vec<Concepto> = req.conceptos
        .iter()
        .map(|c| Concepto::nuevo_con_iva(
            &c.clave_prod_serv,
            &c.clave_unidad,
            c.cantidad,
            &c.descripcion,
            c.valor_unitario,
        ))
        .collect();

    // Genera el CFDI
    let cfdi = Cfdi::nuevo_ingreso(
        req.serie.as_deref(),
        req.folio.as_deref(),
        &req.lugar_expedicion,
        emisor,
        receptor,
        conceptos,
    );

    let sub_total = cfdi.sub_total;
    let total     = cfdi.total;

    // Genera el XML
    let xml = generar_xml(&cfdi)?;

    // Genera la cadena original y hash
    let cadena = generar_cadena_original(&xml)?;
    let hash   = hash_sha256(&cadena);

    Ok(Json(EmitirCfdiResponse {
        xml,
        cadena_original: cadena,
        hash_sha256: hash,
        sub_total,
        total,
        estado: "generado".to_string(),
    }))
}

/// GET /api/v1/cfdi
/// Lista los CFDIs del emisor autenticado
pub async fn listar_cfdi(
    Extension(user): Extension<AuthUser>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "emisor": user.rfc,
        "cfdis": [],
        "total": 0,
        "mensaje": "Base de datos pendiente de conectar — Paso 15"
    }))
}