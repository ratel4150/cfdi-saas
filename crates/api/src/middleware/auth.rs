use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub org: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub rfc: String,
    pub org: String,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match token {
        Some(t) => t,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": {
                        "codigo": "UNAUTHORIZED",
                        "mensaje": "Token JWT requerido — Authorization: Bearer <token>"
                    }
                })),
            ).into_response();
        }
    };

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.inner.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );

    match claims {
        Ok(data) => {
            req.extensions_mut().insert(AuthUser {
                rfc: data.claims.sub,
                org: data.claims.org,
            });
            next.run(req).await
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": {
                    "codigo": "TOKEN_INVALIDO",
                    "mensaje": "Token JWT inválido o expirado"
                }
            })),
        ).into_response(),
    }
}