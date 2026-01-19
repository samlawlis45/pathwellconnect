use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::receipt::{Receipt, ReceiptV2, ExternalEvent, EventType, TrustEvent, TrustEventType};

/// Get the hash of the most recent receipt for hash chaining
pub async fn get_latest_receipt_hash(pool: &PgPool) -> Result<Option<String>> {
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT receipt_hash FROM receipts ORDER BY timestamp DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|row| row.0))
}

/// Store receipt hash for quick lookup (backwards compatibility)
pub async fn store_receipt_hash(
    pool: &PgPool,
    receipt_id: Uuid,
    receipt_hash: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO receipts (receipt_id, receipt_hash, timestamp) VALUES ($1, $2, NOW())
         ON CONFLICT (receipt_id) DO NOTHING"
    )
    .bind(receipt_id)
    .bind(receipt_hash)
    .execute(pool)
    .await?;

    Ok(())
}

/// Create or update a trace record
pub async fn upsert_trace(pool: &PgPool, receipt: &Receipt) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO traces (
            trace_id, correlation_id, started_at, last_event_at, status,
            event_count, policy_deny_count, initiating_agent_id,
            initiating_developer_id, initiating_enterprise_id
        ) VALUES ($1, $2, $3, $3, 'active', 0, 0, $4, $5, $6)
        ON CONFLICT (trace_id) DO NOTHING
        "#
    )
    .bind(receipt.trace_id)
    .bind(&receipt.correlation_id)
    .bind(receipt.timestamp)
    .bind(&receipt.agent_id)
    .bind(&receipt.identity_result.developer_id)
    .bind(&receipt.identity_result.enterprise_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Store a full receipt event
pub async fn store_receipt_event(pool: &PgPool, receipt: &Receipt) -> Result<()> {
    let event_type_str = match receipt.event_type {
        EventType::GatewayRequest => "gateway_request",
        EventType::PolicyEvaluation => "policy_evaluation",
        EventType::IdentityValidation => "identity_validation",
        EventType::ExternalEvent => "external_event",
        EventType::HumanAction => "human_action",
    };

    let full_receipt = serde_json::to_value(receipt)?;
    let headers_json = serde_json::to_value(&receipt.request.headers)?;

    sqlx::query(
        r#"
        INSERT INTO receipt_events (
            receipt_id, trace_id, correlation_id, span_id, parent_span_id,
            timestamp, event_type, event_source_system, event_source_service, event_source_version,
            agent_id, developer_id, enterprise_id,
            request_method, request_path, request_headers, request_body_hash,
            policy_allowed, policy_version, policy_evaluation_ms, identity_valid,
            metadata, full_receipt, receipt_hash, previous_receipt_hash
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10,
            $11, $12, $13,
            $14, $15, $16, $17,
            $18, $19, $20, $21,
            $22, $23, $24, $25
        )
        "#
    )
    .bind(receipt.receipt_id)
    .bind(receipt.trace_id)
    .bind(&receipt.correlation_id)
    .bind(receipt.span_id)
    .bind(receipt.parent_span_id)
    .bind(receipt.timestamp)
    .bind(event_type_str)
    .bind(&receipt.event_source.system)
    .bind(&receipt.event_source.service)
    .bind(&receipt.event_source.version)
    .bind(&receipt.agent_id)
    .bind(&receipt.identity_result.developer_id)
    .bind(&receipt.identity_result.enterprise_id)
    .bind(&receipt.request.method)
    .bind(&receipt.request.path)
    .bind(&headers_json)
    .bind(&receipt.request.body_hash)
    .bind(receipt.policy_result.allowed)
    .bind(&receipt.policy_result.policy_version)
    .bind(receipt.policy_result.evaluation_time_ms as i32)
    .bind(receipt.identity_result.valid)
    .bind(&receipt.metadata)
    .bind(&full_receipt)
    .bind(&receipt.receipt_hash)
    .bind(&receipt.previous_receipt_hash)
    .execute(pool)
    .await?;

    Ok(())
}

/// Store an external event
pub async fn store_external_event(pool: &PgPool, event: &ExternalEvent) -> Result<()> {
    let actor_type = event.actor.as_ref().map(|a| format!("{:?}", a.actor_type).to_lowercase());
    let actor_id = event.actor.as_ref().map(|a| a.actor_id.clone());
    let actor_display_name = event.actor.as_ref().and_then(|a| a.display_name.clone());

    sqlx::query(
        r#"
        INSERT INTO external_events (
            event_id, trace_id, correlation_id,
            event_type, source_system, source_id, timestamp,
            actor_type, actor_id, actor_display_name,
            payload, metadata
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        "#
    )
    .bind(event.event_id)
    .bind(event.trace_id)
    .bind(&event.correlation_id)
    .bind(&event.event_type)
    .bind(&event.source_system)
    .bind(&event.source_id)
    .bind(event.timestamp)
    .bind(actor_type)
    .bind(actor_id)
    .bind(actor_display_name)
    .bind(&event.payload)
    .bind(&event.metadata)
    .execute(pool)
    .await?;

    Ok(())
}

// ========================================
// V2 Storage Functions (Phase 1)
// ========================================

/// Create or update a trace record with trust metrics (v2)
pub async fn upsert_trace_v2(pool: &PgPool, receipt: &ReceiptV2) -> Result<()> {
    let trust_score = receipt.trust_snapshot.as_ref().map(|ts| {
        Decimal::try_from(ts.composite_score).unwrap_or(Decimal::new(5, 1))
    });

    sqlx::query(
        r#"
        INSERT INTO traces (
            trace_id, correlation_id, started_at, last_event_at, status,
            event_count, policy_deny_count, initiating_agent_id,
            initiating_developer_id, initiating_enterprise_id,
            tenant_id, min_trust_score, avg_trust_score, trust_violations
        ) VALUES ($1, $2, $3, $3, 'active', 0, 0, $4, $5, $6, $7, $8, $8, 0)
        ON CONFLICT (trace_id) DO UPDATE SET
            last_event_at = EXCLUDED.last_event_at,
            min_trust_score = LEAST(traces.min_trust_score, EXCLUDED.min_trust_score),
            avg_trust_score = (COALESCE(traces.avg_trust_score, 0) * traces.event_count + COALESCE(EXCLUDED.avg_trust_score, 0)) / (traces.event_count + 1)
        "#
    )
    .bind(receipt.trace_id)
    .bind(&receipt.correlation_id)
    .bind(receipt.timestamp)
    .bind(&receipt.agent_id)
    .bind(&receipt.identity_result.developer_id)
    .bind(&receipt.identity_result.enterprise_id)
    .bind(receipt.tenant_id)
    .bind(trust_score)
    .execute(pool)
    .await?;

    Ok(())
}

/// Store a full receipt event with trust and attribution (v2)
pub async fn store_receipt_event_v2(pool: &PgPool, receipt: &ReceiptV2) -> Result<()> {
    let event_type_str = match receipt.event_type {
        EventType::GatewayRequest => "gateway_request",
        EventType::PolicyEvaluation => "policy_evaluation",
        EventType::IdentityValidation => "identity_validation",
        EventType::ExternalEvent => "external_event",
        EventType::HumanAction => "human_action",
    };

    let full_receipt = serde_json::to_value(receipt)?;
    let headers_json = serde_json::to_value(&receipt.request.headers)?;

    // Extract trust score as decimal
    let trust_score = receipt.trust_snapshot.as_ref().map(|ts| {
        Decimal::try_from(ts.composite_score).unwrap_or(Decimal::new(5, 1))
    });

    // Extract trust dimensions as JSON
    let trust_dimensions = receipt.trust_snapshot.as_ref().map(|ts| {
        serde_json::to_value(&ts.dimensions).unwrap_or(serde_json::Value::Null)
    });

    // Extract attribution as JSON
    let attribution = receipt.attribution_snapshot.as_ref().map(|attr| {
        serde_json::to_value(attr).unwrap_or(serde_json::Value::Null)
    });

    sqlx::query(
        r#"
        INSERT INTO receipt_events (
            receipt_id, trace_id, correlation_id, span_id, parent_span_id,
            timestamp, event_type, event_source_system, event_source_service, event_source_version,
            agent_id, developer_id, enterprise_id,
            request_method, request_path, request_headers, request_body_hash,
            policy_allowed, policy_version, policy_evaluation_ms, identity_valid,
            metadata, full_receipt, receipt_hash, previous_receipt_hash,
            tenant_id, trust_score_at_event, trust_dimensions_at_event, attribution
        ) VALUES (
            $1, $2, $3, $4, $5,
            $6, $7, $8, $9, $10,
            $11, $12, $13,
            $14, $15, $16, $17,
            $18, $19, $20, $21,
            $22, $23, $24, $25,
            $26, $27, $28, $29
        )
        "#
    )
    .bind(receipt.receipt_id)
    .bind(receipt.trace_id)
    .bind(&receipt.correlation_id)
    .bind(receipt.span_id)
    .bind(receipt.parent_span_id)
    .bind(receipt.timestamp)
    .bind(event_type_str)
    .bind(&receipt.event_source.system)
    .bind(&receipt.event_source.service)
    .bind(&receipt.event_source.version)
    .bind(&receipt.agent_id)
    .bind(&receipt.identity_result.developer_id)
    .bind(&receipt.identity_result.enterprise_id)
    .bind(&receipt.request.method)
    .bind(&receipt.request.path)
    .bind(&headers_json)
    .bind(&receipt.request.body_hash)
    .bind(receipt.policy_result.allowed)
    .bind(&receipt.policy_result.policy_version)
    .bind(receipt.policy_result.evaluation_time_ms as i32)
    .bind(receipt.identity_result.valid)
    .bind(&receipt.metadata)
    .bind(&full_receipt)
    .bind(&receipt.receipt_hash)
    .bind(&receipt.previous_receipt_hash)
    .bind(receipt.tenant_id)
    .bind(trust_score)
    .bind(trust_dimensions)
    .bind(attribution)
    .execute(pool)
    .await?;

    Ok(())
}

/// Store a trust event for auditing
pub async fn store_trust_event(pool: &PgPool, event: &TrustEvent) -> Result<()> {
    let event_type_str = match event.event_type {
        TrustEventType::ScoreChecked => "score_checked",
        TrustEventType::ThresholdViolation => "threshold_violation",
        TrustEventType::TrustWarning => "trust_warning",
        TrustEventType::ScoreUpdated => "score_updated",
    };

    let previous_score = event.previous_score.map(|s| {
        Decimal::try_from(s).unwrap_or(Decimal::new(5, 1))
    });

    let new_score = Decimal::try_from(event.new_score).unwrap_or(Decimal::new(5, 1));
    let threshold = Decimal::try_from(event.threshold).unwrap_or(Decimal::new(3, 1));

    sqlx::query(
        r#"
        INSERT INTO trust_events (
            event_id, trace_id, agent_id, event_type, timestamp,
            previous_score, new_score, threshold, passed, action_taken, details
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#
    )
    .bind(event.event_id)
    .bind(event.trace_id)
    .bind(&event.agent_id)
    .bind(event_type_str)
    .bind(event.timestamp)
    .bind(previous_score)
    .bind(new_score)
    .bind(threshold)
    .bind(event.passed)
    .bind(&event.action_taken)
    .bind(&event.details)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get trust events for a trace
pub async fn get_trust_events_for_trace(pool: &PgPool, trace_id: Uuid) -> Result<Vec<TrustEvent>> {
    let rows: Vec<TrustEventRow> = sqlx::query_as(
        r#"
        SELECT event_id, trace_id, agent_id, event_type, timestamp,
               previous_score, new_score, threshold, passed, action_taken, details
        FROM trust_events
        WHERE trace_id = $1
        ORDER BY timestamp ASC
        "#
    )
    .bind(trace_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|row| row.into()).collect())
}

#[derive(Debug, sqlx::FromRow)]
struct TrustEventRow {
    event_id: Uuid,
    trace_id: Uuid,
    agent_id: String,
    event_type: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    previous_score: Option<Decimal>,
    new_score: Decimal,
    threshold: Decimal,
    passed: bool,
    action_taken: Option<String>,
    details: serde_json::Value,
}

impl From<TrustEventRow> for TrustEvent {
    fn from(row: TrustEventRow) -> Self {
        let event_type = match row.event_type.as_str() {
            "threshold_violation" => TrustEventType::ThresholdViolation,
            "trust_warning" => TrustEventType::TrustWarning,
            "score_updated" => TrustEventType::ScoreUpdated,
            _ => TrustEventType::ScoreChecked,
        };

        TrustEvent {
            event_id: row.event_id,
            trace_id: row.trace_id,
            agent_id: row.agent_id,
            event_type,
            timestamp: row.timestamp,
            previous_score: row.previous_score.and_then(|d| d.to_f64()),
            new_score: row.new_score.to_f64().unwrap_or(0.5),
            threshold: row.threshold.to_f64().unwrap_or(0.3),
            passed: row.passed,
            action_taken: row.action_taken,
            details: row.details,
        }
    }
}

/// Update trace trust violations count
pub async fn increment_trust_violations(pool: &PgPool, trace_id: Uuid) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE traces SET trust_violations = COALESCE(trust_violations, 0) + 1
        WHERE trace_id = $1
        "#
    )
    .bind(trace_id)
    .execute(pool)
    .await?;

    Ok(())
}
