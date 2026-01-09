use anyhow::Result;
use tracing::info;
use tracing_subscriber;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};

mod receipt;
mod kafka_producer;
mod s3_archiver;
mod db;
mod store;
mod api;
mod queries;

use api::{
    store_receipt, ingest_external_event,
    list_traces, get_trace, get_trace_timeline, get_trace_decisions, lookup_by_correlation,
};
use store::ReceiptStore;
use kafka_producer::KafkaProducer;
use s3_archiver::S3Archiver;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let kafka_brokers = std::env::var("KAFKA_BROKERS")
        .unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka_topic = std::env::var("KAFKA_TOPIC")
        .unwrap_or_else(|_| "pathwell-receipts".to_string());

    let s3_bucket = std::env::var("S3_BUCKET")
        .unwrap_or_else(|_| "pathwell-receipts".to_string());
    let s3_region = std::env::var("S3_REGION")
        .unwrap_or_else(|_| "us-east-1".to_string());

    let database_url = std::env::var("DATABASE_URL");
    let db_pool = if let Ok(url) = database_url {
        Some(sqlx::PgPool::connect(&url).await?)
    } else {
        None
    };

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3003".to_string())
        .parse::<u16>()
        .unwrap_or(3003);

    info!("Starting Receipt Store service on port {}", port);
    info!("Kafka brokers: {}, topic: {}", kafka_brokers, kafka_topic);
    info!("S3 bucket: {}, region: {}", s3_bucket, s3_region);

    // Initialize Kafka producer
    let kafka = KafkaProducer::new(&kafka_brokers, &kafka_topic)?;
    info!("Kafka producer initialized");

    // Initialize S3 archiver
    let s3 = S3Archiver::new(&s3_bucket, &s3_region).await?;
    info!("S3 archiver initialized");

    // Create receipt store
    let store = Arc::new(ReceiptStore::new(kafka, s3, db_pool));

    // CORS layer for dashboard
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router with all endpoints
    let app = Router::new()
        // Write endpoints
        .route("/v1/receipts", post(store_receipt))
        .route("/v1/events/external", post(ingest_external_event))
        // Read endpoints
        .route("/v1/traces", get(list_traces))
        .route("/v1/traces/:trace_id", get(get_trace))
        .route("/v1/traces/:trace_id/timeline", get(get_trace_timeline))
        .route("/v1/traces/:trace_id/decisions", get(get_trace_decisions))
        .route("/v1/lookup/:correlation_id", get(lookup_by_correlation))
        // Health check
        .route("/health", get(health_check))
        .layer(cors)
        .with_state(store);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Receipt Store listening on 0.0.0.0:{}", port);
    info!("API endpoints:");
    info!("  POST /v1/receipts - Store receipt");
    info!("  POST /v1/events/external - Ingest external event");
    info!("  GET  /v1/traces - List traces");
    info!("  GET  /v1/traces/:trace_id - Get trace detail");
    info!("  GET  /v1/traces/:trace_id/timeline - Get timeline");
    info!("  GET  /v1/traces/:trace_id/decisions - Get decision tree");
    info!("  GET  /v1/lookup/:correlation_id - Lookup by correlation ID");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

