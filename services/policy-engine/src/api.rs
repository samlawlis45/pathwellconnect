use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::engine::{
    PolicyEngine, PolicyRequest, PolicyRequestV2,
    AgentInfoV2, PolicyContext, TrustContext, TrustDimensions,
    AttributionContext, TenantGovernance,
    TrustEvaluationResult, PolicyWarning,
};

// ========================================
// V1 API Types
// ========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateRequest {
    pub agent: crate::engine::AgentInfo,
    pub request: crate::engine::RequestInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResponse {
    pub allowed: bool,
    pub reason: String,
    pub evaluation_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

// ========================================
// V2 API Types (Phase 1)
// ========================================

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateRequestV2 {
    pub agent: AgentInfoV2Request,
    pub request: crate::engine::RequestInfo,
    #[serde(default)]
    pub context: PolicyContextRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentInfoV2Request {
    pub valid: bool,
    pub revoked: bool,
    pub agent_id: String,
    pub developer_id: String,
    pub enterprise_id: Option<String>,
    // Phase 1 additions
    pub tenant_id: Option<String>,
    pub tenant_hierarchy_path: Option<Vec<String>>,
    pub trust_score: Option<TrustContextRequest>,
    pub attribution: Option<AttributionContextRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrustContextRequest {
    pub composite_score: f64,
    #[serde(default)]
    pub dimensions: TrustDimensionsRequest,
    pub threshold: Option<f64>,
    pub threshold_action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TrustDimensionsRequest {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributionContextRequest {
    pub creator_id: Option<String>,
    pub publisher_id: Option<String>,
    #[serde(default = "default_visibility")]
    pub audit_visibility_scope: String,
}

fn default_visibility() -> String {
    "tenant".to_string()
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PolicyContextRequest {
    pub trace_id: Option<String>,
    pub correlation_id: Option<String>,
    pub tenant_governance: Option<TenantGovernanceRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantGovernanceRequest {
    #[serde(default = "default_policy_scope")]
    pub policy_scope: String,
    pub custom_policies: Option<Vec<String>>,
    pub trust_threshold_override: Option<f64>,
}

fn default_policy_scope() -> String {
    "inherit".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResponseV2 {
    pub allowed: bool,
    pub reason: String,
    pub evaluation_time_ms: u64,
    // Phase 1 additions
    pub trust_evaluation: Option<TrustEvaluationResult>,
    pub tenant_policy_applied: Option<String>,
    #[serde(default)]
    pub warnings: Vec<PolicyWarning>,
}

// ========================================
// V1 Handler
// ========================================

pub async fn evaluate_policy(
    State(engine): State<Arc<dyn PolicyEngine>>,
    Json(payload): Json<EvaluateRequest>,
) -> Result<Json<EvaluateResponse>, (StatusCode, Json<ErrorResponse>)> {
    let request = PolicyRequest {
        agent: payload.agent,
        request: payload.request,
    };

    let response = engine.evaluate(&request).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "policy_evaluation_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    Ok(Json(EvaluateResponse {
        allowed: response.allowed,
        reason: response.reason,
        evaluation_time_ms: response.evaluation_time_ms,
    }))
}

// ========================================
// V2 Handler (Phase 1)
// ========================================

pub async fn evaluate_policy_v2(
    State(engine): State<Arc<dyn PolicyEngine>>,
    Json(payload): Json<EvaluateRequestV2>,
) -> Result<Json<EvaluateResponseV2>, (StatusCode, Json<ErrorResponse>)> {
    // Convert request to internal types
    let trust_score = payload.agent.trust_score.map(|ts| TrustContext {
        composite_score: ts.composite_score,
        dimensions: TrustDimensions {
            behavior: ts.dimensions.behavior,
            validation: ts.dimensions.validation,
            provenance: ts.dimensions.provenance,
            alignment: ts.dimensions.alignment,
            reputation: ts.dimensions.reputation,
        },
        threshold: ts.threshold,
        threshold_action: ts.threshold_action,
    });

    let attribution = payload.agent.attribution.map(|attr| AttributionContext {
        creator_id: attr.creator_id,
        publisher_id: attr.publisher_id,
        audit_visibility_scope: attr.audit_visibility_scope,
    });

    let tenant_governance = payload.context.tenant_governance.map(|tg| TenantGovernance {
        policy_scope: tg.policy_scope,
        custom_policies: tg.custom_policies,
        trust_threshold_override: tg.trust_threshold_override,
    });

    let request = PolicyRequestV2 {
        agent: AgentInfoV2 {
            valid: payload.agent.valid,
            revoked: payload.agent.revoked,
            agent_id: payload.agent.agent_id,
            developer_id: payload.agent.developer_id,
            enterprise_id: payload.agent.enterprise_id,
            tenant_id: payload.agent.tenant_id,
            tenant_hierarchy_path: payload.agent.tenant_hierarchy_path,
            trust_score,
            attribution,
        },
        request: payload.request,
        context: PolicyContext {
            trace_id: payload.context.trace_id,
            correlation_id: payload.context.correlation_id,
            tenant_governance,
        },
    };

    let response = engine.evaluate_v2(&request).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "policy_evaluation_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    Ok(Json(EvaluateResponseV2 {
        allowed: response.allowed,
        reason: response.reason,
        evaluation_time_ms: response.evaluation_time_ms,
        trust_evaluation: response.trust_evaluation,
        tenant_policy_applied: response.tenant_policy_applied,
        warnings: response.warnings,
    }))
}

