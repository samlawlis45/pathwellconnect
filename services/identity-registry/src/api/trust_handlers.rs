use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::api::models::*;
use crate::api::routes::AppState;
use crate::db::models::{TrustScore, TrustScoreHistory, TrustDimensionScores};

pub async fn get_trust_score(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
) -> Result<Json<TrustScoreResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    let score = sqlx::query_as!(
        TrustScore,
        r#"
        SELECT
            id, entity_type, entity_id, composite_score, confidence_level,
            dimension_scores, calculation_version, last_calculated_at,
            calculation_inputs, minimum_threshold, threshold_action,
            created_at, updated_at
        FROM trust_scores
        WHERE entity_type = $1 AND entity_id = $2
        "#,
        entity_type,
        entity_id
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

    let score = score.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "trust_score_not_found".to_string(),
                message: format!("Trust score for {} {} not found", entity_type, entity_id),
            }),
        )
    })?;

    let dimensions: TrustDimensionScores =
        serde_json::from_value(score.dimension_scores.clone()).unwrap_or_default();
    let composite = score.composite_score.to_f64().unwrap_or(0.5);
    let threshold = score.minimum_threshold.and_then(|t| t.to_f64());

    Ok(Json(TrustScoreResponse {
        entity_type: score.entity_type,
        entity_id: score.entity_id,
        composite_score: composite,
        confidence_level: score.confidence_level.to_f64().unwrap_or(0.5),
        dimensions: dimensions.into(),
        threshold_status: TrustThresholdStatus {
            minimum_threshold: threshold,
            is_above_threshold: threshold.map(|t| composite >= t).unwrap_or(true),
            action_if_below: score.threshold_action,
        },
        last_calculated_at: score.last_calculated_at.and_utc().to_rfc3339(),
    }))
}

pub async fn create_trust_score(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
    Json(payload): Json<CreateTrustScoreRequest>,
) -> Result<Json<TrustScoreResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    // Check if already exists
    let existing = sqlx::query!(
        "SELECT id FROM trust_scores WHERE entity_type = $1 AND entity_id = $2",
        entity_type,
        entity_id
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
                error: "trust_score_exists".to_string(),
                message: format!("Trust score for {} {} already exists", entity_type, entity_id),
            }),
        ));
    }

    // Build initial dimensions
    let dimensions = if let Some(ref init) = payload.initial_dimensions {
        TrustDimensionScores {
            behavior: init.behavior.unwrap_or(0.5),
            validation: init.validation.unwrap_or(0.5),
            provenance: init.provenance.unwrap_or(0.5),
            alignment: init.alignment.unwrap_or(0.5),
            reputation: init.reputation.unwrap_or(0.5),
        }
    } else {
        TrustDimensionScores::default()
    };

    let composite = dimensions.calculate_composite();
    let dimension_json = serde_json::to_value(&dimensions).unwrap_or_default();
    let threshold = payload.minimum_threshold.map(|t| Decimal::try_from(t).unwrap_or_default());

    let id = Uuid::new_v4();
    let now = Utc::now().naive_utc();

    let score = sqlx::query_as!(
        TrustScore,
        r#"
        INSERT INTO trust_scores (
            id, entity_type, entity_id, composite_score, confidence_level,
            dimension_scores, calculation_version, last_calculated_at,
            minimum_threshold, threshold_action, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING
            id, entity_type, entity_id, composite_score, confidence_level,
            dimension_scores, calculation_version, last_calculated_at,
            calculation_inputs, minimum_threshold, threshold_action,
            created_at, updated_at
        "#,
        id,
        entity_type,
        entity_id,
        Decimal::try_from(composite).unwrap_or_default(),
        Decimal::try_from(0.5).unwrap_or_default(), // Initial confidence
        dimension_json,
        "v1.0.0",
        now,
        threshold,
        payload.threshold_action,
        now,
        now
    )
    .fetch_one(pool)
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

    let composite = score.composite_score.to_f64().unwrap_or(0.5);
    let threshold = score.minimum_threshold.and_then(|t| t.to_f64());

    Ok(Json(TrustScoreResponse {
        entity_type: score.entity_type,
        entity_id: score.entity_id,
        composite_score: composite,
        confidence_level: score.confidence_level.to_f64().unwrap_or(0.5),
        dimensions: dimensions.into(),
        threshold_status: TrustThresholdStatus {
            minimum_threshold: threshold,
            is_above_threshold: threshold.map(|t| composite >= t).unwrap_or(true),
            action_if_below: score.threshold_action,
        },
        last_calculated_at: score.last_calculated_at.and_utc().to_rfc3339(),
    }))
}

pub async fn update_trust_dimension(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
    Json(payload): Json<UpdateTrustDimensionRequest>,
) -> Result<Json<TrustScoreResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    // Get current score
    let current = sqlx::query_as!(
        TrustScore,
        r#"
        SELECT
            id, entity_type, entity_id, composite_score, confidence_level,
            dimension_scores, calculation_version, last_calculated_at,
            calculation_inputs, minimum_threshold, threshold_action,
            created_at, updated_at
        FROM trust_scores
        WHERE entity_type = $1 AND entity_id = $2
        "#,
        entity_type,
        entity_id
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
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "trust_score_not_found".to_string(),
                message: format!("Trust score for {} {} not found", entity_type, entity_id),
            }),
        )
    })?;

    // Parse current dimensions
    let mut dimensions: TrustDimensionScores =
        serde_json::from_value(current.dimension_scores.clone()).unwrap_or_default();

    // Apply delta to specified dimension
    match payload.dimension.to_lowercase().as_str() {
        "behavior" => dimensions.behavior = (dimensions.behavior + payload.delta).clamp(0.0, 1.0),
        "validation" => {
            dimensions.validation = (dimensions.validation + payload.delta).clamp(0.0, 1.0)
        }
        "provenance" => {
            dimensions.provenance = (dimensions.provenance + payload.delta).clamp(0.0, 1.0)
        }
        "alignment" => {
            dimensions.alignment = (dimensions.alignment + payload.delta).clamp(0.0, 1.0)
        }
        "reputation" => {
            dimensions.reputation = (dimensions.reputation + payload.delta).clamp(0.0, 1.0)
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "invalid_dimension".to_string(),
                    message: format!("Unknown dimension: {}", payload.dimension),
                }),
            ))
        }
    }

    let new_composite = dimensions.calculate_composite();
    let dimension_json = serde_json::to_value(&dimensions).unwrap_or_default();
    let now = Utc::now().naive_utc();

    // Record history
    sqlx::query!(
        r#"
        INSERT INTO trust_score_history (
            id, trust_score_id, composite_score, dimension_scores,
            change_reason, change_event_id, recorded_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        Uuid::new_v4(),
        current.id,
        current.composite_score,
        current.dimension_scores,
        Some(payload.reason.clone()),
        payload.event_id,
        now
    )
    .execute(pool)
    .await
    .ok(); // Best effort - don't fail if history insert fails

    // Update score
    let score = sqlx::query_as!(
        TrustScore,
        r#"
        UPDATE trust_scores SET
            composite_score = $3,
            dimension_scores = $4,
            last_calculated_at = $5,
            updated_at = $5
        WHERE entity_type = $1 AND entity_id = $2
        RETURNING
            id, entity_type, entity_id, composite_score, confidence_level,
            dimension_scores, calculation_version, last_calculated_at,
            calculation_inputs, minimum_threshold, threshold_action,
            created_at, updated_at
        "#,
        entity_type,
        entity_id,
        Decimal::try_from(new_composite).unwrap_or_default(),
        dimension_json,
        now
    )
    .fetch_one(pool)
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

    let composite = score.composite_score.to_f64().unwrap_or(0.5);
    let threshold = score.minimum_threshold.and_then(|t| t.to_f64());

    Ok(Json(TrustScoreResponse {
        entity_type: score.entity_type,
        entity_id: score.entity_id,
        composite_score: composite,
        confidence_level: score.confidence_level.to_f64().unwrap_or(0.5),
        dimensions: dimensions.into(),
        threshold_status: TrustThresholdStatus {
            minimum_threshold: threshold,
            is_above_threshold: threshold.map(|t| composite >= t).unwrap_or(true),
            action_if_below: score.threshold_action,
        },
        last_calculated_at: score.last_calculated_at.and_utc().to_rfc3339(),
    }))
}

pub async fn get_trust_score_history(
    State(state): State<AppState>,
    Path((entity_type, entity_id)): Path<(String, Uuid)>,
) -> Result<Json<TrustScoreHistoryResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = &state.pool;

    // Get trust score id
    let score = sqlx::query!("SELECT id FROM trust_scores WHERE entity_type = $1 AND entity_id = $2", entity_type, entity_id)
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
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "trust_score_not_found".to_string(),
                    message: format!("Trust score for {} {} not found", entity_type, entity_id),
                }),
            )
        })?;

    let history = sqlx::query_as!(
        TrustScoreHistory,
        r#"
        SELECT id, trust_score_id, composite_score, dimension_scores,
               change_reason, change_event_id, recorded_at
        FROM trust_score_history
        WHERE trust_score_id = $1
        ORDER BY recorded_at DESC
        LIMIT 100
        "#,
        score.id
    )
    .fetch_all(pool)
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

    Ok(Json(TrustScoreHistoryResponse {
        entries: history
            .into_iter()
            .map(|h| {
                let dims: TrustDimensionScores =
                    serde_json::from_value(h.dimension_scores).unwrap_or_default();
                TrustScoreHistoryEntry {
                    composite_score: h.composite_score.to_f64().unwrap_or(0.5),
                    dimension_scores: dims.into(),
                    change_reason: h.change_reason,
                    recorded_at: h.recorded_at.and_utc().to_rfc3339(),
                }
            })
            .collect(),
    }))
}
