use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::api::handlers;
use crate::pki::CertificateAuthority;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub ca: CertificateAuthority,
}

pub fn create_router(pool: PgPool, ca: CertificateAuthority) -> Router {
    let state = AppState { pool, ca };
    Router::new()
        .route("/v1/developers/register", post(handlers::register_developer))
        .route("/v1/agents/register", post(handlers::register_agent))
        .route("/v1/agents/:agent_id/validate", get(handlers::validate_agent))
        .route("/v1/agents/:agent_id/revoke", post(handlers::revoke_agent))
        .route("/health", get(health_check))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

