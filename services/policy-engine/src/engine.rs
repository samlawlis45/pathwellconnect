use anyhow::Result;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

// ========================================
// V1 Types (Backward Compatible)
// ========================================

/// Policy evaluation request (v1)
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

/// Policy evaluation response (v1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResponse {
    pub allowed: bool,
    pub reason: String,
    pub evaluation_time_ms: u64,
}

// ========================================
// V2 Types (Phase 1 - Trust & Tenant Aware)
// ========================================

/// Enhanced policy evaluation request with trust and tenant context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRequestV2 {
    pub agent: AgentInfoV2,
    pub request: RequestInfo,
    #[serde(default)]
    pub context: PolicyContext,
}

/// Enhanced agent info with trust score and tenant context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfoV2 {
    pub valid: bool,
    pub revoked: bool,
    pub agent_id: String,
    pub developer_id: String,
    pub enterprise_id: Option<String>,
    // Phase 1 additions
    pub tenant_id: Option<String>,
    pub tenant_hierarchy_path: Option<Vec<String>>,
    pub trust_score: Option<TrustContext>,
    pub attribution: Option<AttributionContext>,
}

/// Trust score context for policy evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustContext {
    pub composite_score: f64,
    pub dimensions: TrustDimensions,
    pub threshold: Option<f64>,
    pub threshold_action: Option<String>,
}

/// Trust dimensions breakdown
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrustDimensions {
    #[serde(default = "default_trust_value")]
    pub behavior: f64,
    #[serde(default = "default_trust_value")]
    pub validation: f64,
    #[serde(default = "default_trust_value")]
    pub provenance: f64,
    #[serde(default = "default_trust_value")]
    pub alignment: f64,
    #[serde(default = "default_trust_value")]
    pub reputation: f64,
}

fn default_trust_value() -> f64 {
    0.5
}

/// Attribution context for policy evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributionContext {
    pub creator_id: Option<String>,
    pub publisher_id: Option<String>,
    pub audit_visibility_scope: String,
}

/// Policy evaluation context (tenant governance, trace info)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyContext {
    pub trace_id: Option<String>,
    pub correlation_id: Option<String>,
    pub tenant_governance: Option<TenantGovernance>,
}

/// Tenant governance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantGovernance {
    pub policy_scope: String, // 'inherit', 'override', 'merge'
    pub custom_policies: Option<Vec<String>>,
    pub trust_threshold_override: Option<f64>,
}

/// Enhanced policy evaluation response with trust evaluation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResponseV2 {
    pub allowed: bool,
    pub reason: String,
    pub evaluation_time_ms: u64,
    // Phase 1 additions
    pub trust_evaluation: Option<TrustEvaluationResult>,
    pub tenant_policy_applied: Option<String>,
    #[serde(default)]
    pub warnings: Vec<PolicyWarning>,
}

/// Trust evaluation result details
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

// ========================================
// Policy Engine Trait
// ========================================

/// Trait for pluggable policy engines
#[async_trait]
pub trait PolicyEngine: Send + Sync {
    async fn evaluate(&self, request: &PolicyRequest) -> Result<PolicyResponse>;
    async fn evaluate_v2(&self, request: &PolicyRequestV2) -> Result<PolicyResponseV2>;
}

// ========================================
// OPA Policy Engine Implementation
// ========================================

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
    /// V1 evaluation (backward compatible)
    async fn evaluate(&self, request: &PolicyRequest) -> Result<PolicyResponse> {
        let start = std::time::Instant::now();

        // Prepare OPA input
        let opa_input = serde_json::json!({
            "input": {
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
            }
        });

        // Call OPA API (v1 policy)
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

    /// V2 evaluation with trust and tenant context
    async fn evaluate_v2(&self, request: &PolicyRequestV2) -> Result<PolicyResponseV2> {
        let start = std::time::Instant::now();

        // Build trust score object for OPA
        let trust_score_json = request.agent.trust_score.as_ref().map(|ts| {
            serde_json::json!({
                "composite_score": ts.composite_score,
                "dimensions": {
                    "behavior": ts.dimensions.behavior,
                    "validation": ts.dimensions.validation,
                    "provenance": ts.dimensions.provenance,
                    "alignment": ts.dimensions.alignment,
                    "reputation": ts.dimensions.reputation,
                },
                "threshold": ts.threshold,
                "threshold_action": ts.threshold_action,
            })
        });

        // Build tenant governance object for OPA
        let tenant_governance_json = request.context.tenant_governance.as_ref().map(|tg| {
            serde_json::json!({
                "policy_scope": tg.policy_scope,
                "custom_policies": tg.custom_policies,
                "trust_threshold_override": tg.trust_threshold_override,
            })
        });

        // Prepare OPA input for v2 policy
        let opa_input = serde_json::json!({
            "input": {
                "agent": {
                    "valid": request.agent.valid,
                    "revoked": request.agent.revoked,
                    "agent_id": request.agent.agent_id,
                    "developer_id": request.agent.developer_id,
                    "enterprise_id": request.agent.enterprise_id,
                    "tenant_id": request.agent.tenant_id,
                    "tenant_hierarchy_path": request.agent.tenant_hierarchy_path,
                    "trust_score": trust_score_json,
                    "attribution": request.agent.attribution,
                },
                "request": {
                    "method": request.request.method,
                    "path": request.request.path,
                    "headers": request.request.headers,
                    "body_hash": request.request.body_hash,
                },
                "context": {
                    "trace_id": request.context.trace_id,
                    "correlation_id": request.context.correlation_id,
                    "tenant_governance": tenant_governance_json,
                }
            }
        });

        // Call OPA API (v2 policy) - query multiple rules
        let url = format!("{}/v1/data/pathwell/authz/v2", self.opa_url);
        let response = self
            .client
            .post(&url)
            .json(&opa_input)
            .send()
            .await?;

        let evaluation_time = start.elapsed().as_millis() as u64;

        if !response.status().is_success() {
            return Ok(PolicyResponseV2 {
                allowed: false,
                reason: format!("OPA evaluation failed: {}", response.status()),
                evaluation_time_ms: evaluation_time,
                trust_evaluation: None,
                tenant_policy_applied: None,
                warnings: vec![],
            });
        }

        let opa_result: serde_json::Value = response.json().await?;
        let result = opa_result.get("result").unwrap_or(&serde_json::Value::Null);

        // Extract policy decision
        let allowed = result.get("allow").and_then(|r| r.as_bool()).unwrap_or(false);
        let trust_action = result.get("trust_action").and_then(|r| r.as_str()).map(String::from);
        let applied_threshold = result.get("applied_threshold").and_then(|r| r.as_f64()).unwrap_or(0.3);
        let applied_tenant_policy = result.get("applied_tenant_policy").and_then(|r| r.as_str()).map(String::from);

        // Extract warnings
        let warnings: Vec<PolicyWarning> = result
            .get("warnings")
            .and_then(|w| w.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|w| {
                        Some(PolicyWarning {
                            code: w.get("code")?.as_str()?.to_string(),
                            message: w.get("message")?.as_str()?.to_string(),
                            severity: w.get("severity")?.as_str().unwrap_or("info").to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Build trust evaluation result
        let trust_evaluation = request.agent.trust_score.as_ref().map(|ts| {
            TrustEvaluationResult {
                trust_score_checked: true,
                trust_score: Some(ts.composite_score),
                threshold: applied_threshold,
                passed: ts.composite_score >= applied_threshold,
                action_taken: trust_action.clone(),
            }
        });

        // Determine reason
        let reason = if allowed {
            "Policy allows request".to_string()
        } else if trust_action.as_deref() == Some("block") {
            "Trust score below minimum threshold".to_string()
        } else {
            "Policy denies request".to_string()
        };

        Ok(PolicyResponseV2 {
            allowed,
            reason,
            evaluation_time_ms: evaluation_time,
            trust_evaluation,
            tenant_policy_applied: applied_tenant_policy,
            warnings,
        })
    }
}

