// File: crates/cfdi-core/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CfdiError {
    #[error("RFC inválido: {0}")]
    RfcInvalido(String),

    #[error("Error al generar XML: {0}")]
    XmlError(String),

    #[error("Error de criptografía: {0}")]
    CryptoError(String),

    #[error("Error de validación SAT: {campo} — {mensaje}")]
    ValidacionSat { campo: String, mensaje: String },

    #[error("Complemento Carta Porte inválido: {0}")]
    CartaPorteError(String),

    #[error("Error al leer CSD: {0}")]
    CsdError(String),
}

pub type CfdiResult<T> = Result<T, CfdiError>;