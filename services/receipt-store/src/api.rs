use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::receipt::{ReceiptRequest, ReceiptRequestV2, ExternalEventRequest, TrustEvent};
use crate::store::ReceiptStore;
use crate::queries::{QueryService, TraceQuery, TraceListResponse, TraceDetailResponse, TimelineEvent, DecisionTree};
use crate::db;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreReceiptResponse {
    pub receipt_id: String,
    pub receipt_hash: String,
    pub trace_id: String,
    pub stored: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalEventResponse {
    pub event_id: String,
    pub trace_id: String,
    pub status: String,
}

/// V2 receipt response with trust context
#[derive(Debug, Serialize, Deserialize)]
pub struct StoreReceiptResponseV2 {
    pub receipt_id: String,
    pub receipt_hash: String,
    pub trace_id: String,
    pub stored: bool,
    pub trust_score: Option<f64>,
    pub trust_action: Option<String>,
    pub warnings: Vec<String>,
}

/// Trust events response
#[derive(Debug, Serialize, Deserialize)]
pub struct TrustEventsResponse {
    pub trace_id: String,
    pub events: Vec<TrustEvent>,
    pub total_violations: i32,
}

// ============= Write Endpoints =============

pub async fn store_receipt(
    State(store): State<Arc<ReceiptStore>>,
    Json(payload): Json<ReceiptRequest>,
) -> Result<Json<StoreReceiptResponse>, (StatusCode, Json<ErrorResponse>)> {
    match store.store_receipt(payload).await {
        Ok(receipt) => Ok(Json(StoreReceiptResponse {
            receipt_id: receipt.receipt_id.to_string(),
            receipt_hash: receipt.receipt_hash.clone(),
            trace_id: receipt.trace_id.to_string(),
            stored: true,
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "storage_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

pub async fn ingest_external_event(
    State(store): State<Arc<ReceiptStore>>,
    Json(payload): Json<ExternalEventRequest>,
) -> Result<Json<ExternalEventResponse>, (StatusCode, Json<ErrorResponse>)> {
    match store.store_external_event(payload).await {
        Ok(event) => Ok(Json(ExternalEventResponse {
            event_id: event.event_id.to_string(),
            trace_id: event.trace_id.to_string(),
            status: "accepted".to_string(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "storage_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

// ============= Read Endpoints =============

pub async fn list_traces(
    State(store): State<Arc<ReceiptStore>>,
    Query(params): Query<TraceQuery>,
) -> Result<Json<TraceListResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = match store.db_pool() {
        Some(p) => p.clone(),
        None => return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "database_unavailable".to_string(),
                message: "Database not configured".to_string(),
            }),
        )),
    };

    let query_service = QueryService::new(pool);

    match query_service.list_traces(params).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "query_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

pub async fn get_trace(
    State(store): State<Arc<ReceiptStore>>,
    Path(trace_id): Path<Uuid>,
) -> Result<Json<TraceDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = match store.db_pool() {
        Some(p) => p.clone(),
        None => return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "database_unavailable".to_string(),
                message: "Database not configured".to_string(),
            }),
        )),
    };

    let query_service = QueryService::new(pool);

    match query_service.get_trace_detail(trace_id).await {
        Ok(Some(response)) => Ok(Json(response)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("Trace {} not found", trace_id),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "query_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

pub async fn get_trace_timeline(
    State(store): State<Arc<ReceiptStore>>,
    Path(trace_id): Path<Uuid>,
) -> Result<Json<Vec<TimelineEvent>>, (StatusCode, Json<ErrorResponse>)> {
    let pool = match store.db_pool() {
        Some(p) => p.clone(),
        None => return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "database_unavailable".to_string(),
                message: "Database not configured".to_string(),
            }),
        )),
    };

    let query_service = QueryService::new(pool);

    match query_service.get_timeline(trace_id).await {
        Ok(timeline) => Ok(Json(timeline)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "query_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

pub async fn get_trace_decisions(
    State(store): State<Arc<ReceiptStore>>,
    Path(trace_id): Path<Uuid>,
) -> Result<Json<DecisionTree>, (StatusCode, Json<ErrorResponse>)> {
    let pool = match store.db_pool() {
        Some(p) => p.clone(),
        None => return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "database_unavailable".to_string(),
                message: "Database not configured".to_string(),
            }),
        )),
    };

    let query_service = QueryService::new(pool);

    match query_service.build_decision_tree(trace_id).await {
        Ok(tree) => Ok(Json(tree)),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "query_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

pub async fn lookup_by_correlation(
    State(store): State<Arc<ReceiptStore>>,
    Path(correlation_id): Path<String>,
) -> Result<Json<TraceDetailResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = match store.db_pool() {
        Some(p) => p.clone(),
        None => return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "database_unavailable".to_string(),
                message: "Database not configured".to_string(),
            }),
        )),
    };

    let query_service = QueryService::new(pool);

    // First find the trace by correlation ID
    let trace = match query_service.get_trace_by_correlation(&correlation_id).await {
        Ok(Some(t)) => t,
        Ok(None) => return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("No trace found with correlation ID: {}", correlation_id),
            }),
        )),
        Err(e) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "query_error".to_string(),
                message: e.to_string(),
            }),
        )),
    };

    // Then get full details
    match query_service.get_trace_detail(trace.trace_id).await {
        Ok(Some(response)) => Ok(Json(response)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "not_found".to_string(),
                message: format!("Trace details not found for correlation ID: {}", correlation_id),
            }),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "query_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

// ============= V2 Endpoints (Phase 1) =============

/// Store a v2 receipt with trust and attribution context
pub async fn store_receipt_v2(
    State(store): State<Arc<ReceiptStore>>,
    Json(payload): Json<ReceiptRequestV2>,
) -> Result<Json<StoreReceiptResponseV2>, (StatusCode, Json<ErrorResponse>)> {
    match store.store_receipt_v2(payload).await {
        Ok(receipt) => {
            let trust_score = receipt.trust_snapshot.as_ref().map(|ts| ts.composite_score);
            let trust_action = receipt.trust_snapshot.as_ref().and_then(|ts| ts.trust_action.clone());
            let warnings: Vec<String> = receipt.policy_result.warnings
                .iter()
                .map(|w| w.message.clone())
                .collect();

            Ok(Json(StoreReceiptResponseV2 {
                receipt_id: receipt.receipt_id.to_string(),
                receipt_hash: receipt.receipt_hash.clone(),
                trace_id: receipt.trace_id.to_string(),
                stored: true,
                trust_score,
                trust_action,
                warnings,
            }))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "storage_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

/// Get trust events for a trace
pub async fn get_trace_trust_events(
    State(store): State<Arc<ReceiptStore>>,
    Path(trace_id): Path<Uuid>,
) -> Result<Json<TrustEventsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let pool = match store.db_pool() {
        Some(p) => p.clone(),
        None => return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "database_unavailable".to_string(),
                message: "Database not configured".to_string(),
            }),
        )),
    };

    match db::get_trust_events_for_trace(&pool, trace_id).await {
        Ok(events) => {
            let total_violations = events
                .iter()
                .filter(|e| !e.passed)
                .count() as i32;

            Ok(Json(TrustEventsResponse {
                trace_id: trace_id.to_string(),
                events,
                total_violations,
            }))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "query_error".to_string(),
                message: e.to_string(),
            }),
        )),
    }
}

