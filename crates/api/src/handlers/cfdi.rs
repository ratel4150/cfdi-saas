use axum::{extract::{Extension, State}, Json};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use cfdi_core::{
    Cfdi, Emisor, Receptor, Concepto, RegimenFiscal, UsoCfdi,
    xml::generador::generar_xml,
    crypto::sello::{generar_cadena_original, hash_sha256},
};
use cfdi_db::{CfdiRepository, EmisorRepository};
use cfdi_db::repositories::cfdi::InsertarCfdiInput;
use crate::{error::{ApiError, ApiResult}, middleware::auth::AuthUser, state::AppState};

#[derive(Debug, Deserialize)]
pub struct EmitirCfdiRequest {
    pub serie:            Option<String>,
    pub folio:            Option<String>,
    pub lugar_expedicion: String,
    pub receptor_rfc:     String,
    pub receptor_nombre:  String,
    pub receptor_cp:      String,
    pub receptor_regimen: String,
    pub conceptos:        Vec<ConceptoRequest>,
    pub con_carta_porte:  bool,
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
    pub id:              String,
    pub xml:             String,
    pub cadena_original: String,
    pub hash_sha256:     String,
    pub sub_total:       f64,
    pub total:           f64,
    pub estado:          String,
}

pub async fn emitir_cfdi(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    Json(req): Json<EmitirCfdiRequest>,
) -> ApiResult<Json<EmitirCfdiResponse>> {

    let emisor = Emisor::new(
        &user.rfc,
        &user.org,
        RegimenFiscal::GeneralLeyPersonasMorales,
    )?;

    let receptor = Receptor::new(
        &req.receptor_rfc,
        &req.receptor_nombre,
        &req.receptor_cp,
        &req.receptor_regimen,
        UsoCfdi::GastosEnGeneral,
    )?;

    let conceptos: Vec<Concepto> = req.conceptos.iter()
        .map(|c| Concepto::nuevo_con_iva(
            &c.clave_prod_serv,
            &c.clave_unidad,
            c.cantidad,
            &c.descripcion,
            c.valor_unitario,
        ))
        .collect();

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
    let xml       = generar_xml(&cfdi)?;
    let cadena    = generar_cadena_original(&xml)?;
    let hash      = hash_sha256(&cadena);

    // Busca o crea el emisor en DB
    let emisor_repo = EmisorRepository::new(state.inner.db.clone());
    let emisor_id = emisor_repo
        .insertar(&user.rfc, &user.org, "601", &req.lugar_expedicion)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Guarda el CFDI en DB
    let cfdi_repo = CfdiRepository::new(state.inner.db.clone());
    let cfdi_id = cfdi_repo
        .insertar(InsertarCfdiInput {
            emisor_id,
            serie:             req.serie.clone(),
            folio:             req.folio.clone(),
            fecha:             Utc::now(),
            tipo_comprobante:  "I".to_string(),
            subtotal:          sub_total,
            total,
            moneda:            "MXN".to_string(),
            receptor_rfc:      req.receptor_rfc.clone(),
            receptor_nombre:   req.receptor_nombre.clone(),
            receptor_cp:       req.receptor_cp.clone(),
            receptor_uso_cfdi: "G03".to_string(),
            xml_generado:      xml.clone(),
            cadena_original:   cadena.clone(),
            tiene_carta_porte: req.con_carta_porte,
        })
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(EmitirCfdiResponse {
        id: cfdi_id.to_string(),
        xml,
        cadena_original: cadena,
        hash_sha256: hash,
        sub_total,
        total,
        estado: "generado".to_string(),
    }))
}

pub async fn listar_cfdi(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let emisor_repo = EmisorRepository::new(state.inner.db.clone());

    let emisor = match emisor_repo.buscar_por_rfc(&user.rfc).await {
        Ok(e) => e,
        Err(_) => {
            return Ok(Json(serde_json::json!({
                "emisor": user.rfc,
                "cfdis": [],
                "total": 0,
            })));
        }
    };

    let cfdi_repo = CfdiRepository::new(state.inner.db.clone());
    let total = cfdi_repo
        .contar_por_emisor(emisor.id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let cfdis = cfdi_repo
        .listar_por_emisor(emisor.id, 1, 20)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "emisor": user.rfc,
        "total":  total,
        "cfdis":  cfdis.iter().map(|c| serde_json::json!({
            "id":               c.id,
            "uuid_sat":         c.uuid_sat,
            "serie":            c.serie,
            "folio":            c.folio,
            "fecha":            c.fecha,
            "tipo":             c.tipo_comprobante,
            "subtotal":         c.subtotal.to_string(),
            "total":            c.total.to_string(),
            "receptor_rfc":     c.receptor_rfc,
            "estado":           c.estado,
            "carta_porte":      c.tiene_carta_porte,
        })).collect::<Vec<_>>(),
    })))
}