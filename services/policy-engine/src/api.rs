use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::engine::{PolicyEngine, PolicyRequest};

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

