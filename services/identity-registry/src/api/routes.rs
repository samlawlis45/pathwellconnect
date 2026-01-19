use axum::{
    routing::{get, post, patch, delete},
    Router,
};
use sqlx::PgPool;

use crate::api::handlers;
use crate::api::tenant_handlers;
use crate::api::trust_handlers;
use crate::pki::CertificateAuthority;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub ca: CertificateAuthority,
}

pub fn create_router(pool: PgPool, ca: CertificateAuthority) -> Router {
    let state = AppState { pool, ca };
    Router::new()
        // Existing routes
        .route("/v1/developers/register", post(handlers::register_developer))
        .route("/v1/agents/register", post(handlers::register_agent))
        .route("/v1/agents/:agent_id/validate", get(handlers::validate_agent))
        .route("/v1/agents/:agent_id/revoke", post(handlers::revoke_agent))
        // V2 agent validation with trust/tenant context
        .route("/v2/agents/:agent_id/validate", get(handlers::validate_agent_v2))
        // Tenant routes (TEN.*)
        .route("/v1/tenants", post(tenant_handlers::create_tenant))
        .route("/v1/tenants/:tenant_id", get(tenant_handlers::get_tenant))
        .route("/v1/tenants/:tenant_id", patch(tenant_handlers::update_tenant))
        .route("/v1/tenants/:tenant_id", delete(tenant_handlers::deactivate_tenant))
        .route("/v1/tenants/:tenant_id/hierarchy", get(tenant_handlers::get_tenant_hierarchy))
        // Trust score routes (TRUST.*)
        .route("/v1/trust/:entity_type/:entity_id", get(trust_handlers::get_trust_score))
        .route("/v1/trust/:entity_type/:entity_id", post(trust_handlers::create_trust_score))
        .route("/v1/trust/:entity_type/:entity_id", patch(trust_handlers::update_trust_dimension))
        .route("/v1/trust/:entity_type/:entity_id/history", get(trust_handlers::get_trust_score_history))
        // Health check
        .route("/health", get(health_check))
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

