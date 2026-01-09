use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyResponse {
    pub allowed: bool,
    pub reason: String,
    pub evaluation_time_ms: u64,
}

pub struct PolicyClient {
    base_url: String,
    client: reqwest::Client,
}

impl PolicyClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn evaluate(
        &self,
        agent_id: &str,
        agent_valid: bool,
        agent_revoked: bool,
        developer_id: Uuid,
        enterprise_id: Option<Uuid>,
        method: &str,
        path: &str,
        headers: &std::collections::HashMap<String, String>,
        body_hash: Option<String>,
    ) -> Result<PolicyResponse> {
        let request = PolicyRequest {
            agent: AgentInfo {
                valid: agent_valid,
                revoked: agent_revoked,
                agent_id: agent_id.to_string(),
                developer_id: developer_id.to_string(),
                enterprise_id: enterprise_id.map(|id| id.to_string()),
            },
            request: RequestInfo {
                method: method.to_string(),
                path: path.to_string(),
                headers: headers.clone(),
                body_hash,
            },
        };

        let url = format!("{}/v1/evaluate", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Policy evaluation failed: {}", response.status());
        }

        let result: PolicyResponse = response.json().await?;
        Ok(result)
    }
}

