// File: crates/cfdi-core/src/models/receptor.rs
use serde::{Deserialize, Serialize};
use crate::error::CfdiResult;
use super::emisor::validar_rfc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receptor {
    pub rfc: String,
    pub nombre: String,
    pub domicilio_fiscal_receptor: String,  // código postal
    pub regimen_fiscal_receptor: String,    // código SAT
    pub uso_cfdi: UsoCfdi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsoCfdi {
    #[serde(rename = "G01")] AdquisicionMercancias,
    #[serde(rename = "G03")] GastosEnGeneral,
    #[serde(rename = "S01")] SinEfectosFiscales,
    #[serde(rename = "CP01")] Pagos,
}

impl UsoCfdi {
    pub fn codigo(&self) -> &str {
        match self {
            Self::AdquisicionMercancias => "G01",
            Self::GastosEnGeneral       => "G03",
            Self::SinEfectosFiscales    => "S01",
            Self::Pagos                 => "CP01",
        }
    }
}

impl Receptor {
    pub fn new(
        rfc: &str,
        nombre: &str,
        cp: &str,
        regimen: &str,
        uso: UsoCfdi,
    ) -> CfdiResult<Self> {
        let rfc = rfc.trim().to_uppercase();
        validar_rfc(&rfc)?;
        Ok(Self {
            rfc,
            nombre: nombre.trim().to_uppercase(),
            domicilio_fiscal_receptor: cp.to_string(),
            regimen_fiscal_receptor: regimen.to_string(),
            uso_cfdi: uso,
        })
    }
}