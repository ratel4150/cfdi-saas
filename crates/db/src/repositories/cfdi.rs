use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::error::DbResult;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CfdiRow {
    pub id:                Uuid,
    pub emisor_id:         Uuid,
    pub uuid_sat:          Option<Uuid>,
    pub serie:             Option<String>,
    pub folio:             Option<String>,
    pub fecha:             chrono::DateTime<Utc>,
    pub tipo_comprobante:  String,
    pub subtotal:          f64,
    pub total:             f64,
    pub moneda:            String,
    pub receptor_rfc:      String,
    pub receptor_nombre:   String,
    pub receptor_cp:       String,
    pub receptor_uso_cfdi: String,
    pub xml_generado:      String,
    pub xml_timbrado:      Option<String>,
    pub cadena_original:   String,
    pub sello:             Option<String>,
    pub estado:            String,
    pub tiene_carta_porte: bool,
    pub created_at:        chrono::DateTime<Utc>,
}

#[derive(Debug)]
pub struct InsertarCfdiInput {
    pub emisor_id:         Uuid,
    pub serie:             Option<String>,
    pub folio:             Option<String>,
    pub fecha:             chrono::DateTime<Utc>,
    pub tipo_comprobante:  String,
    pub subtotal:          f64,
    pub total:             f64,
    pub moneda:            String,
    pub receptor_rfc:      String,
    pub receptor_nombre:   String,
    pub receptor_cp:       String,
    pub receptor_uso_cfdi: String,
    pub xml_generado:      String,
    pub cadena_original:   String,
    pub tiene_carta_porte: bool,
}

pub struct CfdiRepository {
    pub pool: PgPool,
}

impl CfdiRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insertar(&self, input: InsertarCfdiInput) -> DbResult<Uuid> {
        let row = sqlx::query_as::<_, (Uuid,)>(
            r#"INSERT INTO cfdis (
                emisor_id, serie, folio, fecha,
                tipo_comprobante, subtotal, total, moneda,
                receptor_rfc, receptor_nombre, receptor_cp,
                receptor_uso_cfdi, xml_generado, cadena_original,
                tiene_carta_porte, estado
            ) VALUES (
                $1, $2, $3, $4, $5, $6::numeric, $7::numeric, $8,
                $9, $10, $11, $12, $13, $14, $15, 'generado'
            ) RETURNING id"#
        )
        .bind(input.emisor_id)
        .bind(input.serie)
        .bind(input.folio)
        .bind(input.fecha)
        .bind(input.tipo_comprobante)
        .bind(input.subtotal)
        .bind(input.total)
        .bind(input.moneda)
        .bind(input.receptor_rfc)
        .bind(input.receptor_nombre)
        .bind(input.receptor_cp)
        .bind(input.receptor_uso_cfdi)
        .bind(input.xml_generado)
        .bind(input.cadena_original)
        .bind(input.tiene_carta_porte)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }

    pub async fn actualizar_timbrado(
        &self,
        id: Uuid,
        uuid_sat: Uuid,
        xml_timbrado: &str,
        sello_sat: &str,
        no_certificado_sat: &str,
    ) -> DbResult<()> {
        sqlx::query(
            r#"UPDATE cfdis SET
                uuid_sat           = $2,
                xml_timbrado       = $3,
                sello_sat          = $4,
                no_certificado_sat = $5,
                fecha_timbrado     = NOW(),
                estado             = 'timbrado',
                updated_at         = NOW()
            WHERE id = $1"#
        )
        .bind(id)
        .bind(uuid_sat)
        .bind(xml_timbrado)
        .bind(sello_sat)
        .bind(no_certificado_sat)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn marcar_error(&self, id: Uuid, mensaje: &str) -> DbResult<()> {
        sqlx::query(
            "UPDATE cfdis SET estado = 'error', error_mensaje = $2, updated_at = NOW() WHERE id = $1"
        )
        .bind(id)
        .bind(mensaje)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn contar_por_emisor(&self, emisor_id: Uuid) -> DbResult<i64> {
        let row = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM cfdis WHERE emisor_id = $1"
        )
        .bind(emisor_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    pub async fn listar_por_emisor(
        &self,
        emisor_id: Uuid,
        pagina: i64,
        por_pagina: i64,
    ) -> DbResult<Vec<CfdiRow>> {
        let offset = (pagina - 1) * por_pagina;
        let rows = sqlx::query_as::<_, CfdiRow>(
            r#"SELECT
                id, emisor_id, uuid_sat, serie, folio, fecha,
                tipo_comprobante,
                subtotal::float8 as subtotal,
                total::float8 as total,
                moneda, receptor_rfc, receptor_nombre, receptor_cp,
                receptor_uso_cfdi, xml_generado, xml_timbrado,
                cadena_original, sello,
                estado::TEXT as estado,
                tiene_carta_porte, created_at
            FROM cfdis
            WHERE emisor_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3"#
        )
        .bind(emisor_id)
        .bind(por_pagina)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}