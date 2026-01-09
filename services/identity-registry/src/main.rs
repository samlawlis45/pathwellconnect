use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

mod db;
mod pki;
mod api;

use db::create_pool;
use pki::CertificateAuthority;
use api::create_router;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/pathwell".to_string());
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);

    info!("Starting Identity Registry service on port {}", port);
    info!("Connecting to database: {}", database_url);

    // Create database pool
    let pool = create_pool(&database_url).await?;
    
    // Check if tables exist, if so skip migrations
    let tables_exist = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
            SELECT FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'agents'
        )"
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(false);
    
    if !tables_exist {
        info!("Running database migrations...");
        sqlx::migrate!().run(&pool).await?;
        info!("Migrations completed");
    } else {
        info!("Database tables already exist, skipping migrations");
    }

    // Initialize Certificate Authority
    info!("Initializing Certificate Authority...");
    let ca = CertificateAuthority::new()?;
    info!("Certificate Authority initialized");

    // Create router
    let app = create_router(pool, ca);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Identity Registry listening on 0.0.0.0:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

