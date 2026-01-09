use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event types for categorizing receipt events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    GatewayRequest,
    PolicyEvaluation,
    IdentityValidation,
    ExternalEvent,
    HumanAction,
}

/// Source system information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSource {
    pub system: String,
    pub service: String,
    pub version: String,
}

impl Default for EventSource {
    fn default() -> Self {
        Self {
            system: "pathwell".to_string(),
            service: "proxy-gateway".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptRequest {
    pub trace_id: Uuid,
    pub correlation_id: Option<String>,
    pub span_id: Uuid,
    pub parent_span_id: Option<Uuid>,
    pub agent_id: String,
    pub event_type: EventType,
    pub event_source: EventSource,
    pub request: RequestInfo,
    pub policy_result: PolicyResult,
    pub identity_result: IdentityResult,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestInfo {
    pub method: String,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyResult {
    pub allowed: bool,
    pub policy_version: String,
    pub evaluation_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityResult {
    pub valid: bool,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
}

pub struct ReceiptClient {
    base_url: String,
    client: reqwest::Client,
}

impl ReceiptClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn store_receipt(&self, receipt: ReceiptRequest) -> Result<()> {
        let url = format!("{}/v1/receipts", self.base_url);
        // Fire and forget - don't block on receipt storage
        let client = self.client.clone();
        let url_clone = url.clone();
        tokio::spawn(async move {
            if let Err(e) = client.post(&url_clone).json(&receipt).send().await {
                tracing::warn!("Failed to store receipt: {}", e);
            }
        });
        Ok(())
    }
}

