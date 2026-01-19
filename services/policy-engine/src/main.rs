use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;

mod engine;
mod api;

use engine::{OPAEngine, PolicyEngine};
use api::{evaluate_policy, evaluate_policy_v2};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let opa_url = std::env::var("OPA_URL")
        .unwrap_or_else(|_| "http://localhost:8181".to_string());
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3002".to_string())
        .parse::<u16>()
        .unwrap_or(3002);

    info!("Starting Policy Engine service on port {}", port);
    info!("OPA URL: {}", opa_url);

    // Create OPA engine
    let engine: Arc<dyn PolicyEngine> = Arc::new(OPAEngine::new(opa_url));

    // Create router
    let app = Router::new()
        .route("/v1/evaluate", post(evaluate_policy))
        .route("/v2/evaluate", post(evaluate_policy_v2))
        .route("/health", axum::routing::get(health_check))
        .with_state(engine);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Policy Engine listening on 0.0.0.0:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

