use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use hex;

/// Event types for categorizing receipt events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    GatewayRequest,
    PolicyEvaluation,
    IdentityValidation,
    ExternalEvent,
    HumanAction,
}

impl Default for EventType {
    fn default() -> Self {
        EventType::GatewayRequest
    }
}

/// Source system information for tracing event origin
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
            service: "unknown".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

/// Actor types for identifying who performed an action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Agent,
    Human,
    System,
}

/// Actor information for tracking who performed an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorInfo {
    pub actor_type: ActorType,
    pub actor_id: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub receipt_id: Uuid,
    pub trace_id: Uuid,
    pub correlation_id: Option<String>,
    pub span_id: Uuid,
    pub parent_span_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub agent_id: String,
    pub event_type: EventType,
    pub event_source: EventSource,
    pub request: RequestInfo,
    pub policy_result: PolicyResult,
    pub identity_result: IdentityResult,
    pub metadata: Option<serde_json::Value>,
    pub receipt_hash: String,
    pub previous_receipt_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInfo {
    pub method: String,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResult {
    pub allowed: bool,
    pub policy_version: String,
    pub evaluation_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityResult {
    pub valid: bool,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
}

impl Receipt {
    pub fn new(
        trace_id: Uuid,
        correlation_id: Option<String>,
        span_id: Uuid,
        parent_span_id: Option<Uuid>,
        agent_id: String,
        event_type: EventType,
        event_source: EventSource,
        request: RequestInfo,
        policy_result: PolicyResult,
        identity_result: IdentityResult,
        metadata: Option<serde_json::Value>,
        previous_receipt_hash: Option<String>,
    ) -> Self {
        let receipt_id = Uuid::new_v4();
        let timestamp = Utc::now();

        // Create receipt without hash first
        let receipt = Self {
            receipt_id,
            trace_id,
            correlation_id,
            span_id,
            parent_span_id,
            timestamp,
            agent_id,
            event_type,
            event_source,
            request,
            policy_result,
            identity_result,
            metadata,
            receipt_hash: String::new(), // Will be calculated
            previous_receipt_hash,
        };

        // Calculate hash and create final receipt
        let receipt_hash = receipt.calculate_hash();
        Self {
            receipt_hash,
            ..receipt
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        // Hash all fields except receipt_hash itself
        let hash_data = serde_json::json!({
            "receipt_id": self.receipt_id,
            "trace_id": self.trace_id,
            "correlation_id": self.correlation_id,
            "span_id": self.span_id,
            "parent_span_id": self.parent_span_id,
            "timestamp": self.timestamp.to_rfc3339(),
            "agent_id": self.agent_id,
            "event_type": self.event_type,
            "event_source": self.event_source,
            "request": self.request,
            "policy_result": self.policy_result,
            "identity_result": self.identity_result,
            "metadata": self.metadata,
            "previous_receipt_hash": self.previous_receipt_hash,
        });

        hasher.update(serde_json::to_string(&hash_data).unwrap().as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn verify_chain(&self, previous_receipt: &Receipt) -> bool {
        // Verify that previous receipt hash matches
        if let Some(ref prev_hash) = self.previous_receipt_hash {
            if prev_hash != &previous_receipt.receipt_hash {
                return false;
            }
        }
        
        // Verify current receipt hash
        let calculated_hash = self.calculate_hash();
        calculated_hash == self.receipt_hash
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptRequest {
    pub trace_id: Option<Uuid>,
    pub correlation_id: Option<String>,
    pub span_id: Option<Uuid>,
    pub parent_span_id: Option<Uuid>,
    pub agent_id: String,
    pub event_type: Option<EventType>,
    pub event_source: Option<EventSource>,
    pub request: RequestInfo,
    pub policy_result: PolicyResult,
    pub identity_result: IdentityResult,
    pub metadata: Option<serde_json::Value>,
}

/// External event for integration with SAP, Salesforce, etc.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalEventRequest {
    pub trace_id: Uuid,
    pub correlation_id: Option<String>,
    pub event_type: String,
    pub source_system: String,
    pub source_id: String,
    pub timestamp: DateTime<Utc>,
    pub actor: Option<ActorInfo>,
    pub payload: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
}

/// Stored external event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalEvent {
    pub event_id: Uuid,
    pub trace_id: Uuid,
    pub correlation_id: Option<String>,
    pub event_type: String,
    pub source_system: String,
    pub source_id: String,
    pub timestamp: DateTime<Utc>,
    pub actor: Option<ActorInfo>,
    pub payload: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

impl ExternalEvent {
    pub fn from_request(request: ExternalEventRequest) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            trace_id: request.trace_id,
            correlation_id: request.correlation_id,
            event_type: request.event_type,
            source_system: request.source_system,
            source_id: request.source_id,
            timestamp: request.timestamp,
            actor: request.actor,
            payload: request.payload,
            metadata: request.metadata,
            created_at: Utc::now(),
        }
    }
}

