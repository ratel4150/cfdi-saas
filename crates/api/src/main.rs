// File: crates/api/src/main.rs
mod error;
mod handlers;
mod middleware;
mod routes;
mod state;

use state::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Inicializa logging estructurado
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cfdi_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Carga variables de entorno
    dotenvy::dotenv().ok();

    let state = AppState::new();
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