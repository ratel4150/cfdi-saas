// File: crates/db/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Error de base de datos: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Registro no encontrado: {0}")]
    NotFound(String),

    #[error("Error de serialización: {0}")]
    Serialization(String),
}

pub type DbResult<T> = Result<T, DbError>;