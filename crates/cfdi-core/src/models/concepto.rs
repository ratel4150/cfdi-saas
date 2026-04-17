// File: crates/cfdi-core/src/models/concepto.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concepto {
    pub clave_prod_serv: String,    // catálogo SAT c_ClaveProdServ
    pub clave_unidad: String,       // catálogo SAT c_ClaveUnidad
    pub cantidad: f64,
    pub descripcion: String,
    pub valor_unitario: f64,
    pub importe: f64,
    pub objeto_imp: ObjetoImpuesto,
    pub impuestos: Option<Impuestos>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjetoImpuesto {
    #[serde(rename = "01")] NoObjetoImpuesto,
    #[serde(rename = "02")] SiObjetoImpuesto,
    #[serde(rename = "03")] SiObjetoNoObligado,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Impuestos {
    pub traslados: Vec<Traslado>,
    pub retenciones: Vec<Retencion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Traslado {
    pub base: f64,
    pub impuesto: String,      // "002" = IVA, "003" = IEPS
    pub tipo_factor: String,   // "Tasa", "Cuota", "Exento"
    pub tasa_o_cuota: f64,
    pub importe: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retencion {
    pub base: f64,
    pub impuesto: String,
    pub tipo_factor: String,
    pub tasa_o_cuota: f64,
    pub importe: f64,
}

impl Concepto {
    pub fn nuevo_con_iva(
        clave_prod_serv: &str,
        clave_unidad: &str,
        cantidad: f64,
        descripcion: &str,
        valor_unitario: f64,
    ) -> Self {
        let importe = (cantidad * valor_unitario * 100.0).round() / 100.0;
        let base_iva = importe;
        let iva = (base_iva * 0.16 * 100.0).round() / 100.0;

        Self {
            clave_prod_serv: clave_prod_serv.to_string(),
            clave_unidad: clave_unidad.to_string(),
            cantidad,
            descripcion: descripcion.to_string(),
            valor_unitario,
            importe,
            objeto_imp: ObjetoImpuesto::SiObjetoImpuesto,
            impuestos: Some(Impuestos {
                traslados: vec![Traslado {
                    base: base_iva,
                    impuesto: "002".to_string(),
                    tipo_factor: "Tasa".to_string(),
                    tasa_o_cuota: 0.16,
                    importe: iva,
                }],
                retenciones: vec![],
            }),
        }
    }
}