use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::db::models::{TenantType, Attribution, TrustDimensionScores};

// ========================================
// Existing Models
// ========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAgentRequest {
    pub agent_id: String,
    pub developer_id: String,
    pub enterprise_id: Option<String>,
    pub public_key: String,
    // Phase 1 additions
    pub tenant_id: Option<String>,
    pub attribution: Option<AttributionRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAgentResponse {
    pub agent_id: String,
    pub certificate_chain: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateAgentResponse {
    pub valid: bool,
    pub agent_id: String,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
    pub revoked: bool,
}

/// Enhanced validation response with trust and tenant context
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateAgentResponseV2 {
    pub valid: bool,
    pub agent_id: String,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
    pub revoked: bool,
    // Phase 1 additions
    pub tenant_id: Option<Uuid>,
    pub tenant_hierarchy_path: Option<Vec<String>>,
    pub trust_score: Option<TrustScoreSummary>,
    pub attribution: Option<AttributionSummary>,
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
    pub tenant_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDeveloperResponse {
    pub developer_id: String,
    pub created_at: String,
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

// ========================================
// Tenant API Models (TEN.*)
// ========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTenantRequest {
    pub tenant_id: String,
    #[serde(default)]
    pub tenant_type: Option<String>,
    pub display_name: Option<String>,
    pub parent_tenant_id: Option<String>,
    pub governance_config: Option<serde_json::Value>,
    pub visibility_config: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTenantResponse {
    pub id: Uuid,
    pub tenant_id: String,
    pub tenant_type: String,
    pub hierarchy_depth: i32,
    pub hierarchy_path: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantResponse {
    pub id: Uuid,
    pub tenant_id: String,
    pub tenant_type: String,
    pub display_name: Option<String>,
    pub parent_tenant_id: Option<Uuid>,
    pub root_tenant_id: Option<Uuid>,
    pub hierarchy_depth: i32,
    pub hierarchy_path: Option<Vec<String>>,
    pub governance_config: serde_json::Value,
    pub visibility_config: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantHierarchyResponse {
    pub tenant: TenantSummary,
    pub ancestors: Vec<TenantSummary>,
    pub children: Vec<TenantSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantSummary {
    pub id: Uuid,
    pub tenant_id: String,
    pub tenant_type: String,
    pub display_name: Option<String>,
    pub hierarchy_depth: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTenantRequest {
    pub display_name: Option<String>,
    pub governance_config: Option<serde_json::Value>,
    pub visibility_config: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

// ========================================
// Trust Score API Models (TRUST.*)
// ========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustScoreResponse {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub composite_score: f64,
    pub confidence_level: f64,
    pub dimensions: TrustDimensionsResponse,
    pub threshold_status: TrustThresholdStatus,
    pub last_calculated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustDimensionsResponse {
    pub behavior: f64,
    pub validation: f64,
    pub provenance: f64,
    pub alignment: f64,
    pub reputation: f64,
}

impl From<TrustDimensionScores> for TrustDimensionsResponse {
    fn from(scores: TrustDimensionScores) -> Self {
        Self {
            behavior: scores.behavior,
            validation: scores.validation,
            provenance: scores.provenance,
            alignment: scores.alignment,
            reputation: scores.reputation,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustThresholdStatus {
    pub minimum_threshold: Option<f64>,
    pub is_above_threshold: bool,
    pub action_if_below: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustScoreSummary {
    pub composite_score: f64,
    pub is_trusted: bool,
    pub threshold_action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTrustScoreRequest {
    pub minimum_threshold: Option<f64>,
    pub threshold_action: Option<String>,
    pub initial_dimensions: Option<TrustDimensionsRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustDimensionsRequest {
    pub behavior: Option<f64>,
    pub validation: Option<f64>,
    pub provenance: Option<f64>,
    pub alignment: Option<f64>,
    pub reputation: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTrustDimensionRequest {
    pub dimension: String,
    pub delta: f64,
    pub reason: String,
    pub event_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustScoreHistoryResponse {
    pub entries: Vec<TrustScoreHistoryEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustScoreHistoryEntry {
    pub composite_score: f64,
    pub dimension_scores: TrustDimensionsResponse,
    pub change_reason: Option<String>,
    pub recorded_at: String,
}

// ========================================
// Attribution API Models (AUTH.OBJ)
// ========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributionRequest {
    pub creator_id: Option<Uuid>,
    pub publisher_id: Option<Uuid>,
    pub revenue_token: Option<String>,
    pub royalty_distribution_map: Option<serde_json::Value>,
    pub licensing_terms: Option<LicensingTermsRequest>,
    pub attribution_protocol_uri: Option<String>,
    pub audit_visibility_scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicensingTermsRequest {
    pub license_type: String,
    #[serde(default)]
    pub allowed_uses: Vec<String>,
    #[serde(default)]
    pub restrictions: Vec<String>,
    pub expiry: Option<String>,
    pub custom_terms: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributionResponse {
    pub creator_id: Option<Uuid>,
    pub publisher_id: Option<Uuid>,
    pub consumer_chain: Vec<Uuid>,
    pub revenue_token: Option<String>,
    pub royalty_distribution_map: Option<serde_json::Value>,
    pub licensing_terms: Option<serde_json::Value>,
    pub attribution_protocol_uri: Option<String>,
    pub version_lineage: Vec<serde_json::Value>,
    pub audit_visibility_scope: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributionSummary {
    pub creator_id: Option<Uuid>,
    pub publisher_id: Option<Uuid>,
    pub audit_visibility_scope: String,
}

impl From<Attribution> for AttributionSummary {
    fn from(attr: Attribution) -> Self {
        Self {
            creator_id: attr.creator_id,
            publisher_id: attr.publisher_id,
            audit_visibility_scope: format!("{:?}", attr.audit_visibility_scope).to_lowercase(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddConsumerRequest {
    pub consumer_id: Uuid,
    pub trace_id: Option<Uuid>,
}

