use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn crear_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Conectando a PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .connect(database_url)
        .await?;
    tracing::info!("Pool PostgreSQL creado correctamente");
    Ok(pool)
}

pub async fn correr_migraciones(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Verifica que las tablas existen
    let existe = sqlx::query_as::<_, (bool,)>(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'cfdis')"
    )
    .fetch_one(pool)
    .await?;

    if existe.0 {
        tracing::info!("Tablas verificadas correctamente");
    } else {
        tracing::warn!("Tablas no encontradas — corre las migraciones manualmente");
    }

    Ok(())
}