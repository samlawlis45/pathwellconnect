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

// ========================================
// V2 Types (Phase 1 - Trust & Attribution)
// ========================================

/// Trust score context captured at receipt time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustContext {
    pub composite_score: f64,
    pub dimensions: TrustDimensions,
    pub threshold_applied: f64,
    pub trust_action: Option<String>,
}

/// Trust dimensions breakdown
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrustDimensions {
    #[serde(default = "default_trust")]
    pub behavior: f64,
    #[serde(default = "default_trust")]
    pub validation: f64,
    #[serde(default = "default_trust")]
    pub provenance: f64,
    #[serde(default = "default_trust")]
    pub alignment: f64,
    #[serde(default = "default_trust")]
    pub reputation: f64,
}

fn default_trust() -> f64 {
    0.5
}

/// Attribution context for receipt
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttributionContext {
    pub creator_id: Option<String>,
    pub publisher_id: Option<String>,
    pub audit_visibility_scope: Option<String>,
}

/// Enhanced policy result with trust evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResultV2 {
    pub allowed: bool,
    pub policy_version: String,
    pub evaluation_time_ms: u64,
    pub trust_evaluation: Option<TrustEvaluationResult>,
    pub tenant_policy_applied: Option<String>,
    #[serde(default)]
    pub warnings: Vec<PolicyWarning>,
}

/// Trust evaluation result from policy engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEvaluationResult {
    pub trust_score_checked: bool,
    pub trust_score: Option<f64>,
    pub threshold: f64,
    pub passed: bool,
    pub action_taken: Option<String>,
}

/// Policy warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyWarning {
    pub code: String,
    pub message: String,
    pub severity: String,
}

/// Enhanced identity result with tenant context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityResultV2 {
    pub valid: bool,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub tenant_hierarchy_path: Option<Vec<String>>,
    pub trust_score: Option<TrustContext>,
    pub attribution: Option<AttributionContext>,
}

/// V2 Receipt with trust and attribution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptV2 {
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
    pub policy_result: PolicyResultV2,
    pub identity_result: IdentityResultV2,
    pub metadata: Option<serde_json::Value>,
    pub receipt_hash: String,
    pub previous_receipt_hash: Option<String>,
    // Phase 1 additions
    pub tenant_id: Option<Uuid>,
    pub trust_snapshot: Option<TrustContext>,
    pub attribution_snapshot: Option<AttributionContext>,
}

impl ReceiptV2 {
    pub fn new(
        trace_id: Uuid,
        correlation_id: Option<String>,
        span_id: Uuid,
        parent_span_id: Option<Uuid>,
        agent_id: String,
        event_type: EventType,
        event_source: EventSource,
        request: RequestInfo,
        policy_result: PolicyResultV2,
        identity_result: IdentityResultV2,
        metadata: Option<serde_json::Value>,
        previous_receipt_hash: Option<String>,
    ) -> Self {
        let receipt_id = Uuid::new_v4();
        let timestamp = Utc::now();

        // Extract tenant and trust info from identity result
        let tenant_id = identity_result.tenant_id;
        let trust_snapshot = identity_result.trust_score.clone();
        let attribution_snapshot = identity_result.attribution.clone();

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
            receipt_hash: String::new(),
            previous_receipt_hash,
            tenant_id,
            trust_snapshot,
            attribution_snapshot,
        };

        // Calculate hash and create final receipt
        let receipt_hash = receipt.calculate_hash();
        Self {
            receipt_hash,
            ..receipt
        }
    }

    pub fn calculate_hash(&self) -> String {
        use sha2::Digest;
        let mut hasher = Sha256::new();

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
            "tenant_id": self.tenant_id,
            "trust_snapshot": self.trust_snapshot,
            "attribution_snapshot": self.attribution_snapshot,
        });

        hasher.update(serde_json::to_string(&hash_data).unwrap().as_bytes());
        hex::encode(hasher.finalize())
    }
}

/// V2 Receipt request with trust and attribution
#[derive(Debug, Serialize, Deserialize)]
pub struct ReceiptRequestV2 {
    pub trace_id: Option<Uuid>,
    pub correlation_id: Option<String>,
    pub span_id: Option<Uuid>,
    pub parent_span_id: Option<Uuid>,
    pub agent_id: String,
    pub event_type: Option<EventType>,
    pub event_source: Option<EventSource>,
    pub request: RequestInfo,
    pub policy_result: PolicyResultV2,
    pub identity_result: IdentityResultV2,
    pub metadata: Option<serde_json::Value>,
}

/// Trust event for tracking trust score changes over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEvent {
    pub event_id: Uuid,
    pub trace_id: Uuid,
    pub agent_id: String,
    pub event_type: TrustEventType,
    pub timestamp: DateTime<Utc>,
    pub previous_score: Option<f64>,
    pub new_score: f64,
    pub threshold: f64,
    pub passed: bool,
    pub action_taken: Option<String>,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TrustEventType {
    ScoreChecked,
    ThresholdViolation,
    TrustWarning,
    ScoreUpdated,
}

