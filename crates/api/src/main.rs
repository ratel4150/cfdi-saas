mod error;
mod handlers;
mod middleware;
mod routes;
mod state;

use state::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cfdi_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            "postgres://cfdi:cfdi123@localhost:5432/cfdi_db".to_string()
        });

    // Conecta a PostgreSQL
    let pool = cfdi_db::crear_pool(&database_url)
        .await
        .expect("No se pudo conectar a PostgreSQL");

    // Corre migraciones automáticamente
    cfdi_db::correr_migraciones(&pool)
        .await
        .expect("Error al correr migraciones");

    let state = AppState::new(pool).await;
    let app   = routes::crear_router(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("🚀 CFDI API corriendo en http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("No se pudo bindear el puerto");

    axum::serve(listener, app)
        .await
        .expect("Error al correr el servidor");
}