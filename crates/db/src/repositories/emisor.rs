use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::error::{DbError, DbResult};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmisorRow {
    pub id:             Uuid,
    pub rfc:            String,
    pub nombre:         String,
    pub regimen_fiscal: String,
    pub codigo_postal:  String,
    pub activo:         bool,
}

pub struct EmisorRepository {
    pub pool: PgPool,
}

impl EmisorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn buscar_por_rfc(&self, rfc: &str) -> DbResult<EmisorRow> {
        sqlx::query_as::<_, EmisorRow>(
            "SELECT id, rfc, nombre, regimen_fiscal, codigo_postal, activo
             FROM emisores WHERE rfc = $1 AND activo = true"
        )
        .bind(rfc)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| DbError::NotFound(format!("Emisor RFC {} no encontrado", rfc)))
    }

    pub async fn insertar(
        &self,
        rfc: &str,
        nombre: &str,
        regimen_fiscal: &str,
        codigo_postal: &str,
    ) -> DbResult<Uuid> {
        let row = sqlx::query_as::<_, (Uuid,)>(
            r#"INSERT INTO emisores (rfc, nombre, regimen_fiscal, codigo_postal)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (rfc) DO UPDATE SET
                   nombre = EXCLUDED.nombre,
                   updated_at = NOW()
               RETURNING id"#
        )
        .bind(rfc)
        .bind(nombre)
        .bind(regimen_fiscal)
        .bind(codigo_postal)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0)
    }
}