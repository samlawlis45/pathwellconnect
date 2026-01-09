use anyhow::Result;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

/// Policy evaluation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRequest {
    pub agent: AgentInfo,
    pub request: RequestInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub valid: bool,
    pub revoked: bool,
    pub agent_id: String,
    pub developer_id: String,
    pub enterprise_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInfo {
    pub method: String,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body_hash: Option<String>,
}

/// Policy evaluation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResponse {
    pub allowed: bool,
    pub reason: String,
    pub evaluation_time_ms: u64,
}

/// Trait for pluggable policy engines
#[async_trait]
pub trait PolicyEngine: Send + Sync {
    async fn evaluate(&self, request: &PolicyRequest) -> Result<PolicyResponse>;
}

/// OPA Policy Engine implementation
pub struct OPAEngine {
    opa_url: String,
    client: reqwest::Client,
}

impl OPAEngine {
    pub fn new(opa_url: String) -> Self {
        Self {
            opa_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl PolicyEngine for OPAEngine {
    async fn evaluate(&self, request: &PolicyRequest) -> Result<PolicyResponse> {
        let start = std::time::Instant::now();
        
        // Prepare OPA input
        let opa_input = serde_json::json!({
            "agent": {
                "valid": request.agent.valid,
                "revoked": request.agent.revoked,
                "agent_id": request.agent.agent_id,
                "developer_id": request.agent.developer_id,
                "enterprise_id": request.agent.enterprise_id,
            },
            "request": {
                "method": request.request.method,
                "path": request.request.path,
                "headers": request.request.headers,
                "body_hash": request.request.body_hash,
            }
        });

        // Call OPA API
        let url = format!("{}/v1/data/pathwell/authz/allow", self.opa_url);
        let response = self
            .client
            .post(&url)
            .json(&opa_input)
            .send()
            .await?;

        let evaluation_time = start.elapsed().as_millis() as u64;

        if !response.status().is_success() {
            return Ok(PolicyResponse {
                allowed: false,
                reason: format!("OPA evaluation failed: {}", response.status()),
                evaluation_time_ms: evaluation_time,
            });
        }

        let opa_result: serde_json::Value = response.json().await?;
        let allowed = opa_result.get("result").and_then(|r| r.as_bool()).unwrap_or(false);

        Ok(PolicyResponse {
            allowed,
            reason: if allowed {
                "Policy allows request".to_string()
            } else {
                "Policy denies request".to_string()
            },
            evaluation_time_ms: evaluation_time,
        })
    }
}

