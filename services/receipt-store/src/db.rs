use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;

use crate::receipt::{Receipt, ExternalEvent, EventType};

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
