use std::sync::Arc;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<Inner>,
}

pub struct Inner {
    pub jwt_secret: String,
    pub pac_token:  String,
    pub app_env:    String,
    pub db:         PgPool,
}

impl AppState {
    pub async fn new(db: PgPool) -> Self {
        Self {
            inner: Arc::new(Inner {
                jwt_secret: std::env::var("JWT_SECRET")
                    .unwrap_or_else(|_| "dev-secret-256-chars".to_string()),
                pac_token: std::env::var("PAC_TOKEN")
                    .unwrap_or_else(|_| "pac-token-dev".to_string()),
                app_env: std::env::var("APP_ENV")
                    .unwrap_or_else(|_| "development".to_string()),
                db,
            }),
        }
    }
}