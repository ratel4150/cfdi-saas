// File: crates/cfdi-core/src/models/emisor.rs
use serde::{Deserialize, Serialize};
use crate::error::{CfdiError, CfdiResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emisor {
    pub rfc: String,
    pub nombre: String,
    pub regimen_fiscal: RegimenFiscal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegimenFiscal {
    #[serde(rename = "601")]
    GeneralLeyPersonasMorales,
    #[serde(rename = "612")]
    PersonasFisicasActividadesEmpresariales,
    #[serde(rename = "626")]
    SimplificadoConfianza,
}

impl RegimenFiscal {
    pub fn codigo(&self) -> &str {
        match self {
            Self::GeneralLeyPersonasMorales              => "601",
            Self::PersonasFisicasActividadesEmpresariales => "612",
            Self::SimplificadoConfianza                  => "626",
        }
    }
}

impl Emisor {
    pub fn new(rfc: &str, nombre: &str, regimen: RegimenFiscal) -> CfdiResult<Self> {
        let rfc = rfc.trim().to_uppercase();
        validar_rfc(&rfc)?;
        Ok(Self {
            rfc,
            nombre: nombre.trim().to_uppercase(),
            regimen_fiscal: regimen,
        })
    }
}

pub fn validar_rfc(rfc: &str) -> CfdiResult<()> {
    // RFC persona moral:  4 letras + 6 dígitos fecha + 3 homoclave = 13 chars
    // RFC persona física: 4 letras + 6 dígitos fecha + 3 homoclave = 13 chars
    // RFC genérico XAXX010101000 (público en general)
    let len = rfc.len();
    if len != 12 && len != 13 {
        return Err(CfdiError::RfcInvalido(format!(
            "El RFC '{}' debe tener 12 o 13 caracteres, tiene {}",
            rfc, len
        )));
    }
    Ok(())
}