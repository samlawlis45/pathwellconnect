use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAgentRequest {
    pub agent_id: String,
    pub developer_id: String,
    pub enterprise_id: Option<String>,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAgentResponse {
    pub agent_id: String,
    pub certificate_chain: String,
    pub created_at: String, // ISO 8601 string
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateAgentResponse {
    pub valid: bool,
    pub agent_id: String,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
    pub revoked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeAgentRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDeveloperRequest {
    pub developer_id: String,
    pub enterprise_id: Option<String>,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDeveloperResponse {
    pub developer_id: String,
    pub created_at: String, // ISO 8601 string
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub id: Uuid,
    pub agent_id: String,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
    pub public_key: String,
    pub certificate_chain: String,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

