use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub target_backend_url: String,
    pub identity_registry_url: String,
    pub policy_engine_url: String,
    pub receipt_store_url: String,
    pub listen_port: u16,
    pub listen_host: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            target_backend_url: std::env::var("TARGET_BACKEND_URL")
                .expect("TARGET_BACKEND_URL must be set"),
            identity_registry_url: std::env::var("IDENTITY_REGISTRY_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            policy_engine_url: std::env::var("POLICY_ENGINE_URL")
                .unwrap_or_else(|_| "http://localhost:3002".to_string()),
            receipt_store_url: std::env::var("RECEIPT_STORE_URL")
                .unwrap_or_else(|_| "http://localhost:3003".to_string()),
            listen_port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            listen_host: std::env::var("LISTEN_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
        }
    }
}

