use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateAgentResponse {
    pub valid: bool,
    pub agent_id: String,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
    pub revoked: bool,
}

pub struct IdentityClient {
    base_url: String,
    client: reqwest::Client,
}

impl IdentityClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn validate_agent(&self, agent_id: &str) -> Result<ValidateAgentResponse> {
        let url = format!("{}/v1/agents/{}/validate", self.base_url, agent_id);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Identity validation failed: {}", response.status());
        }

        let result: ValidateAgentResponse = response.json().await?;
        Ok(result)
    }
}

