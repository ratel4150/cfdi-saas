// File: crates/db/src/lib.rs
pub mod error;
pub mod pool;
pub mod repositories;

pub use error::{DbError, DbResult};
pub use pool::{crear_pool, correr_migraciones};
pub use repositories::{CfdiRepository, EmisorRepository};