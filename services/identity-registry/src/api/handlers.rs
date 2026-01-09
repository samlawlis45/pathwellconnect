use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use chrono::Utc;

use crate::api::models::*;
use crate::api::routes::AppState;
use crate::db::models::*;

pub async fn register_agent(
    State(state): State<AppState>,
    Json(payload): Json<RegisterAgentRequest>,
) -> Result<Json<RegisterAgentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;
    let ca = &state.ca;
    // Validate developer exists
    let developer = sqlx::query_as!(
        Developer,
        "SELECT id, developer_id, enterprise_id, public_key, created_at, updated_at 
         FROM developers WHERE developer_id = $1",
        payload.developer_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    let developer = developer.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "developer_not_found".to_string(),
                message: format!("Developer {} not found", payload.developer_id),
            }),
        )
    })?;

    // Check if enterprise_id matches if provided
    if let Some(ref enterprise_id) = payload.enterprise_id {
        if let Some(dev_enterprise_id) = developer.enterprise_id {
            if dev_enterprise_id.to_string() != *enterprise_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "enterprise_mismatch".to_string(),
                        message: "Enterprise ID does not match developer's enterprise".to_string(),
                    }),
                ));
            }
        }
    }

    // Check if agent already exists
    let existing = sqlx::query!(
        "SELECT id FROM agents WHERE agent_id = $1",
        payload.agent_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "agent_exists".to_string(),
                message: format!("Agent {} already exists", payload.agent_id),
            }),
        ));
    }

    // Issue certificate
    let certificate_chain = ca
        .issue_agent_certificate(&payload.agent_id, &payload.public_key)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "certificate_error".to_string(),
                    message: e.to_string(),
                }),
            )
        })?;

    // Insert agent
    let now = Utc::now().naive_utc();
    let agent_id_uuid = Uuid::new_v4();
    let enterprise_id_uuid = if let Some(ref eid) = payload.enterprise_id {
        sqlx::query!("SELECT id FROM enterprises WHERE enterprise_id = $1", eid)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
            .map(|row| row.id)
    } else {
        None
    };

    sqlx::query!(
        "INSERT INTO agents (id, agent_id, developer_id, enterprise_id, public_key, certificate_chain, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        agent_id_uuid,
        payload.agent_id,
        developer.id,
        enterprise_id_uuid,
        payload.public_key,
        certificate_chain,
        now,
        now
    )
    .execute(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    Ok(Json(RegisterAgentResponse {
        agent_id: payload.agent_id,
        certificate_chain,
        created_at: Utc::now().to_rfc3339(),
    }))
}

pub async fn validate_agent(
    State(state): State<AppState>,
    Path(agent_id): Path<String>,
) -> Result<Json<ValidateAgentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;
    let agent = sqlx::query_as!(
        Agent,
        "SELECT id, agent_id, developer_id, enterprise_id, public_key, certificate_chain, created_at, revoked_at, updated_at
         FROM agents WHERE agent_id = $1",
        agent_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    let agent = agent.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "agent_not_found".to_string(),
                message: format!("Agent {} not found", agent_id),
            }),
        )
    })?;

    Ok(Json(ValidateAgentResponse {
        valid: agent.revoked_at.is_none(),
        agent_id: agent.agent_id,
        developer_id: agent.developer_id,
        enterprise_id: agent.enterprise_id,
        revoked: agent.revoked_at.is_some(),
    }))
}

pub async fn revoke_agent(
    State(state): State<AppState>,
    Path(agent_id): Path<String>,
    Json(_payload): Json<RevokeAgentRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;
    let now = Utc::now().naive_utc();
    let result = sqlx::query!(
        "UPDATE agents SET revoked_at = $1, updated_at = $1 WHERE agent_id = $2 AND revoked_at IS NULL",
        now,
        agent_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "agent_not_found".to_string(),
                message: format!("Agent {} not found or already revoked", agent_id),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn register_developer(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDeveloperRequest>,
) -> Result<Json<RegisterDeveloperResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    // Check if developer already exists
    let existing = sqlx::query!(
        "SELECT id FROM developers WHERE developer_id = $1",
        payload.developer_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "developer_exists".to_string(),
                message: format!("Developer {} already exists", payload.developer_id),
            }),
        ));
    }

    // Get enterprise_id if provided
    let enterprise_id_uuid = if let Some(ref eid) = payload.enterprise_id {
        sqlx::query!("SELECT id FROM enterprises WHERE enterprise_id = $1", eid)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
            .map(|row| row.id)
    } else {
        None
    };

    // Insert developer
    let now = Utc::now().naive_utc();
    let developer_uuid = Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO developers (id, developer_id, enterprise_id, public_key, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
        developer_uuid,
        payload.developer_id,
        enterprise_id_uuid,
        payload.public_key,
        now,
        now
    )
    .execute(pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "database_error".to_string(),
                message: e.to_string(),
            }),
        )
    })?;

    Ok(Json(RegisterDeveloperResponse {
        developer_id: payload.developer_id,
        created_at: Utc::now().to_rfc3339(),
    }))
}

