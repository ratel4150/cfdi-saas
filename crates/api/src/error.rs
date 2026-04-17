// File: crates/api/src/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("No autorizado: {0}")]
    Unauthorized(String),

    #[error("Solicitud inválida: {0}")]
    BadRequest(String),

    #[error("Error interno: {0}")]
    Internal(String),

    #[error("Error del motor CFDI: {0}")]
    Cfdi(#[from] cfdi_core::CfdiError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, codigo, mensaje) = match &self {
            ApiError::Unauthorized(m) => (StatusCode::UNAUTHORIZED,   "UNAUTHORIZED",    m.clone()),
            ApiError::BadRequest(m)   => (StatusCode::BAD_REQUEST,    "BAD_REQUEST",     m.clone()),
            ApiError::Internal(m)     => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL", m.clone()),
            ApiError::Cfdi(e)         => (StatusCode::BAD_REQUEST,    "CFDI_ERROR",      e.to_string()),
        };

        (status, Json(json!({
            "error": {
                "codigo":  codigo,
                "mensaje": mensaje,
            }
        }))).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;