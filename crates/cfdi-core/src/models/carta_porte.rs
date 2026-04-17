// File: crates/cfdi-core/src/models/carta_porte.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartaPorte {
    pub version: String,               // "3.1"
    pub trans_internac: String,        // "No" o "Sí"
    pub total_dist_rec: f64,           // km totales del recorrido
    pub ubicaciones: Vec<Ubicacion>,
    pub mercancias: Mercancias,
    pub figura_transporte: Vec<FiguraTransporte>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ubicacion {
    pub tipo_ubicacion: TipoUbicacion,
    pub id_ubicacion: String,       // "OR000001" origen, "DE000001" destino
    pub rfc_remitente_dest: String,
    pub nombre_remitente_dest: Option<String>,
    pub fecha_hora_salida_llegada: String,
    pub distancia_recorrida: Option<f64>,
    pub domicilio: Domicilio,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoUbicacion {
    #[serde(rename = "Origen")]  Origen,
    #[serde(rename = "Destino")] Destino,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domicilio {
    pub calle: Option<String>,
    pub municipio: String,
    pub estado: String,           // catálogo c_Estado SAT
    pub pais: String,             // "MEX"
    pub codigo_postal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mercancias {
    pub peso_bruto_total: f64,
    pub unidad_peso: String,      // "KGM" kilogramos
    pub num_total_mercancias: u32,
    pub cargo_por_tasacion: Option<f64>,
    pub mercancias: Vec<Mercancia>,
    pub autotransporte: Option<Autotransporte>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mercancia {
    pub bienes_transp: String,        // clave SAT c_ClaveProdServCP
    pub descripcion: String,
    pub cantidad: f64,
    pub clave_unidad: String,         // c_ClaveUnidad SAT
    pub peso_en_kg: f64,
    pub valor_mercancia: f64,
    pub moneda: String,               // "MXN"
    pub material_peligroso: Option<String>,
    pub cve_material_peligroso: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Autotransporte {
    pub perm_sct: String,             // permiso SCT: "TPAF01" etc
    pub num_permiso_sct: String,
    pub config_vehicular: String,     // "C2" camión 2 ejes
    pub placa_vm: String,
    pub anio_modelo_vm: u16,
    pub seguro_resp_civil: SeguroRespCivil,
    pub remolques: Vec<Remolque>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeguroRespCivil {
    pub asegura_resp_civil: String,
    pub poliza_resp_civil: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remolque {
    pub sub_tipo_rem: String,    // "CTR001" etc
    pub placa: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiguraTransporte {
    pub tipo_figura: TipoFigura,
    pub rfc_figura: String,
    pub num_licencia: Option<String>,
    pub nombre_figura: String,
    pub domicilio_fiscal_figura: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipoFigura {
    #[serde(rename = "01")] Operador,
    #[serde(rename = "02")] Propietario,
    #[serde(rename = "03")] Arrendador,
}

impl CartaPorte {
    pub fn nuevo_autotransporte(
        ubicaciones: Vec<Ubicacion>,
        mercancias: Mercancias,
        operador_rfc: &str,
        operador_nombre: &str,
        num_licencia: &str,
    ) -> Self {
        Self {
            version: "3.1".to_string(),
            trans_internac: "No".to_string(),
            total_dist_rec: ubicaciones.iter()
                .filter_map(|u| u.distancia_recorrida)
                .sum(),
            ubicaciones,
            mercancias,
            figura_transporte: vec![FiguraTransporte {
                tipo_figura: TipoFigura::Operador,
                rfc_figura: operador_rfc.to_uppercase(),
                num_licencia: Some(num_licencia.to_string()),
                nombre_figura: operador_nombre.to_uppercase(),
                domicilio_fiscal_figura: None,
            }],
        }
    }
}