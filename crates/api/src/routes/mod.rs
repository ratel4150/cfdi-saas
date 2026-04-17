// File: crates/api/src/routes/mod.rs
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::{
    handlers::{health, cfdi},
    middleware::auth::auth_middleware,
    state::AppState,
};

pub fn crear_router(state: AppState) -> Router {
    // Rutas públicas — sin auth
    let publicas = Router::new()
        .route("/",       get(health::root))
        .route("/health", get(health::health));

    // Rutas protegidas — requieren JWT
    let protegidas = Router::new()
        .route("/api/v1/cfdi",        get(cfdi::listar_cfdi))
        .route("/api/v1/cfdi/emitir", post(cfdi::emitir_cfdi))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(publicas)
        .merge(protegidas)
        .with_state(state)
}