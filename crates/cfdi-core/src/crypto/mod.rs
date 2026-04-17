// File: crates/cfdi-core/src/crypto/mod.rs
// Módulo de criptografía — siguiente paso
pub mod sello;
pub use sello::{generar_cadena_original, firmar_cadena, hash_sha256};