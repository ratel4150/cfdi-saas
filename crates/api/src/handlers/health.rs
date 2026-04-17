// File: crates/api/src/handlers/health.rs
use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::state::AppState;

pub async fn health(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "app":    "CFDI Carta Porte SAT",
        "version": "1.0.0",
        "env":    state.inner.app_env,
    }))
}

pub async fn root() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "app":    "CFDI Carta Porte SAT 4.0",
        "docs":   "/api/v1",
    }))
}