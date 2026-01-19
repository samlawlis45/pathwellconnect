use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDateTime};
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

// ========================================
// Existing Models
// ========================================

#[derive(Debug, Clone, FromRow)]
pub struct Enterprise {
    pub id: Uuid,
    pub enterprise_id: String,
    pub public_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // Phase 1 additions
    pub tenant_id: Option<Uuid>,
    pub trust_score_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Developer {
    pub id: Uuid,
    pub developer_id: String,
    pub enterprise_id: Option<Uuid>,
    pub public_key: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // Phase 1 additions
    pub tenant_id: Option<Uuid>,
    pub trust_score_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub agent_id: String,
    pub developer_id: Uuid,
    pub enterprise_id: Option<Uuid>,
    pub public_key: String,
    pub certificate_chain: String,
    pub created_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
    // Phase 1 additions
    pub tenant_id: Option<Uuid>,
    pub attribution: serde_json::Value,
    pub trust_score_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

// ========================================
// Tenant Hierarchy Models (TEN.*)
// ========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tenant_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TenantType {
    Platform,
    Parent,
    Child,
    Instance,
}

impl Default for TenantType {
    fn default() -> Self {
        TenantType::Child
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tenant_relationship_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TenantRelationshipType {
    Owns,
    Governs,
    Delegates,
    Observes,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub tenant_id: String,
    pub tenant_type: TenantType,
    pub display_name: Option<String>,
    pub parent_tenant_id: Option<Uuid>,
    pub root_tenant_id: Option<Uuid>,
    pub hierarchy_depth: i32,
    pub hierarchy_path: Option<Vec<String>>,
    pub governance_config: serde_json::Value,
    pub visibility_config: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deactivated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TenantRelationship {
    pub id: Uuid,
    pub source_tenant_id: Uuid,
    pub target_tenant_id: Uuid,
    pub relationship_type: TenantRelationshipType,
    pub permissions: serde_json::Value,
    pub constraints: Option<serde_json::Value>,
    pub valid_from: NaiveDateTime,
    pub valid_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ========================================
// Trust Models (TRUST.*)
// ========================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrustDimensionScores {
    pub behavior: f64,
    pub validation: f64,
    pub provenance: f64,
    pub alignment: f64,
    pub reputation: f64,
}

impl TrustDimensionScores {
    pub fn new() -> Self {
        Self {
            behavior: 0.5,
            validation: 0.5,
            provenance: 0.5,
            alignment: 0.5,
            reputation: 0.5,
        }
    }

    pub fn calculate_composite(&self) -> f64 {
        // Equal weighting by default
        (self.behavior + self.validation + self.provenance + self.alignment + self.reputation) / 5.0
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TrustScore {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub composite_score: Decimal,
    pub confidence_level: Decimal,
    pub dimension_scores: serde_json::Value,
    pub calculation_version: String,
    pub last_calculated_at: NaiveDateTime,
    pub calculation_inputs: Option<serde_json::Value>,
    pub minimum_threshold: Option<Decimal>,
    pub threshold_action: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TrustScoreHistory {
    pub id: Uuid,
    pub trust_score_id: Uuid,
    pub composite_score: Decimal,
    pub dimension_scores: serde_json::Value,
    pub change_reason: Option<String>,
    pub change_event_id: Option<Uuid>,
    pub recorded_at: NaiveDateTime,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TrustVaultEntry {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub public_key_hash: String,
    pub key_algorithm: String,
    pub key_purpose: String,
    pub certificate_fingerprint: Option<String>,
    pub certificate_chain_hash: Option<String>,
    pub issuer_fingerprint: Option<String>,
    pub verification_status: String,
    pub last_verified_at: Option<NaiveDateTime>,
    pub verification_method: Option<String>,
    pub valid_from: NaiveDateTime,
    pub valid_until: Option<NaiveDateTime>,
    pub revoked_at: Option<NaiveDateTime>,
    pub revocation_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ========================================
// Trust Risk Models (TRUST.RISK)
// ========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "risk_severity", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "risk_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RiskStatus {
    Open,
    Investigating,
    Mitigated,
    Resolved,
    Accepted,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TrustRiskEvent {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub risk_type: String,
    pub severity: RiskSeverity,
    pub status: RiskStatus,
    pub description: String,
    pub evidence: Option<serde_json::Value>,
    pub impact_assessment: Option<serde_json::Value>,
    pub mitigation_actions: Option<serde_json::Value>,
    pub mitigated_at: Option<NaiveDateTime>,
    pub mitigated_by: Option<String>,
    pub related_event_ids: Option<Vec<Uuid>>,
    pub trace_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub resolved_at: Option<NaiveDateTime>,
}

// ========================================
// Attribution Models (AUTH.OBJ)
// ========================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<Uuid>,
    #[serde(default)]
    pub consumer_chain: Vec<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty_distribution_map: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licensing_terms: Option<LicensingTerms>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_protocol_uri: Option<String>,
    #[serde(default)]
    pub version_lineage: Vec<VersionLineageEntry>,
    #[serde(default)]
    pub audit_visibility_scope: AuditVisibilityScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensingTerms {
    pub license_type: String,
    #[serde(default)]
    pub allowed_uses: Vec<String>,
    #[serde(default)]
    pub restrictions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_terms: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionLineageEntry {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Uuid>,
    pub timestamp: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_summary: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditVisibilityScope {
    Public,
    #[default]
    Tenant,
    Private,
}

