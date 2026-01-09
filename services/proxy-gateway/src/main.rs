use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use axum::{
    body::Body,
    extract::{Request, State, Path},
    http::{Response, StatusCode},
    routing::any,
    Router,
};
use std::sync::Arc;

mod config;
mod interceptor;
mod identity_client;
mod policy_client;
mod receipt_client;

use config::Config;
use interceptor::Interceptor;

async fn handle_all(
    State(interceptor): State<Arc<Interceptor>>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    // Extract body bytes and request parts
    let (parts, body) = req.into_parts();
    let body_bytes = axum::body::to_bytes(body, usize::MAX).await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // axum and hyper both use http::request::Parts, so we can pass directly
    // Pass body bytes directly to interceptor
    match interceptor.intercept(parts, hyper::body::Bytes::from(body_bytes)).await {
        Ok(resp) => {
            let (parts, body) = resp.into_parts();
            Ok(Response::from_parts(parts, Body::from(body)))
        }
        Err(e) => {
            error!("Request handling error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    
    info!("Starting Proxy Gateway");
    info!("Listening on {}:{}", config.listen_host, config.listen_port);
    info!("Target backend: {}", config.target_backend_url);
    info!("Identity Registry: {}", config.identity_registry_url);
    info!("Policy Engine: {}", config.policy_engine_url);
    info!("Receipt Store: {}", config.receipt_store_url);

    let interceptor = Arc::new(Interceptor::new(config.clone()));

    let app = Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .fallback(handle_all)
        .with_state(interceptor);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.listen_host, config.listen_port)).await?;
    info!("Proxy Gateway listening on {}:{}", config.listen_host, config.listen_port);

    axum::serve(listener, app).await?;

    Ok(())
}
