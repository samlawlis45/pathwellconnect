use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::receipt::{
    Receipt, ReceiptRequest, EventSource, ExternalEvent, ExternalEventRequest,
    ReceiptV2, ReceiptRequestV2, TrustEvent, TrustEventType,
};
use crate::kafka_producer::KafkaProducer;
use crate::s3_archiver::S3Archiver;
use crate::db;

pub struct ReceiptStore {
    kafka: KafkaProducer,
    s3: S3Archiver,
    db_pool: Option<PgPool>,
}

impl ReceiptStore {
    pub fn new(
        kafka: KafkaProducer,
        s3: S3Archiver,
        db_pool: Option<PgPool>,
    ) -> Self {
        Self { kafka, s3, db_pool }
    }

    pub async fn store_receipt(&self, request: ReceiptRequest) -> Result<Receipt> {
        // Get previous receipt hash for chain
        let previous_hash = if let Some(ref pool) = self.db_pool {
            db::get_latest_receipt_hash(pool).await?
        } else {
            None
        };

        // Generate or use provided trace context
        let trace_id = request.trace_id.unwrap_or_else(Uuid::new_v4);
        let span_id = request.span_id.unwrap_or_else(Uuid::new_v4);
        let event_type = request.event_type.unwrap_or_default();
        let event_source = request.event_source.unwrap_or_else(|| EventSource {
            system: "pathwell".to_string(),
            service: "proxy-gateway".to_string(),
            version: "1.0.0".to_string(),
        });

        // Create receipt with hash chain and trace context
        let receipt = Receipt::new(
            trace_id,
            request.correlation_id.clone(),
            span_id,
            request.parent_span_id,
            request.agent_id,
            event_type,
            event_source,
            request.request,
            request.policy_result,
            request.identity_result,
            request.metadata,
            previous_hash,
        );

        // Serialize receipt
        let receipt_json = serde_json::to_string(&receipt)?;

        // Store in database if available
        if let Some(ref pool) = self.db_pool {
            // Ensure trace exists (create or update)
            db::upsert_trace(pool, &receipt).await?;

            // Store full receipt event
            db::store_receipt_event(pool, &receipt).await?;

            // Store hash for chain verification (backwards compatibility)
            db::store_receipt_hash(pool, receipt.receipt_id, &receipt.receipt_hash).await?;
        }

        // Send to Kafka (non-blocking, best effort)
        if let Err(e) = self.kafka.send_receipt(&receipt_json).await {
            tracing::warn!("Failed to send receipt to Kafka: {}", e);
        }

        // Archive to S3 (non-blocking, best effort)
        if let Err(e) = self.s3.archive_receipt(&receipt_json).await {
            tracing::warn!("Failed to archive receipt to S3: {}", e);
        }

        Ok(receipt)
    }

    pub async fn store_external_event(&self, request: ExternalEventRequest) -> Result<ExternalEvent> {
        let event = ExternalEvent::from_request(request);

        if let Some(ref pool) = self.db_pool {
            db::store_external_event(pool, &event).await?;
        }

        // Also send to Kafka for streaming consumers
        let event_json = serde_json::to_string(&event)?;
        if let Err(e) = self.kafka.send_receipt(&event_json).await {
            tracing::warn!("Failed to send external event to Kafka: {}", e);
        }

        Ok(event)
    }

    pub fn db_pool(&self) -> Option<&PgPool> {
        self.db_pool.as_ref()
    }

    /// Store a v2 receipt with trust and attribution context
    pub async fn store_receipt_v2(&self, request: ReceiptRequestV2) -> Result<ReceiptV2> {
        // Get previous receipt hash for chain
        let previous_hash = if let Some(ref pool) = self.db_pool {
            db::get_latest_receipt_hash(pool).await?
        } else {
            None
        };

        // Generate or use provided trace context
        let trace_id = request.trace_id.unwrap_or_else(Uuid::new_v4);
        let span_id = request.span_id.unwrap_or_else(Uuid::new_v4);
        let event_type = request.event_type.unwrap_or_default();
        let event_source = request.event_source.unwrap_or_else(|| EventSource {
            system: "pathwell".to_string(),
            service: "proxy-gateway".to_string(),
            version: "2.0.0".to_string(),
        });

        // Create v2 receipt with trust and attribution
        let receipt = ReceiptV2::new(
            trace_id,
            request.correlation_id.clone(),
            span_id,
            request.parent_span_id,
            request.agent_id.clone(),
            event_type,
            event_source,
            request.request,
            request.policy_result.clone(),
            request.identity_result.clone(),
            request.metadata,
            previous_hash,
        );

        // Serialize receipt
        let receipt_json = serde_json::to_string(&receipt)?;

        // Store in database if available
        if let Some(ref pool) = self.db_pool {
            // Ensure trace exists (create or update with trust metrics)
            db::upsert_trace_v2(pool, &receipt).await?;

            // Store full receipt event with trust/attribution
            db::store_receipt_event_v2(pool, &receipt).await?;

            // Store hash for chain verification
            db::store_receipt_hash(pool, receipt.receipt_id, &receipt.receipt_hash).await?;

            // If there was a trust evaluation, store trust event
            if let Some(ref trust_eval) = request.policy_result.trust_evaluation {
                let trust_event = TrustEvent {
                    event_id: Uuid::new_v4(),
                    trace_id,
                    agent_id: request.agent_id.clone(),
                    event_type: if !trust_eval.passed {
                        TrustEventType::ThresholdViolation
                    } else if request.policy_result.warnings.iter().any(|w| w.code.starts_with("TRUST_")) {
                        TrustEventType::TrustWarning
                    } else {
                        TrustEventType::ScoreChecked
                    },
                    timestamp: Utc::now(),
                    previous_score: None,
                    new_score: trust_eval.trust_score.unwrap_or(0.5),
                    threshold: trust_eval.threshold,
                    passed: trust_eval.passed,
                    action_taken: trust_eval.action_taken.clone(),
                    details: serde_json::json!({
                        "warnings": request.policy_result.warnings,
                        "tenant_policy": request.policy_result.tenant_policy_applied,
                    }),
                };
                db::store_trust_event(pool, &trust_event).await?;

                // Increment trust violations if threshold was not passed
                if !trust_eval.passed {
                    db::increment_trust_violations(pool, trace_id).await?;
                }
            }
        }

        // Send to Kafka (non-blocking, best effort)
        if let Err(e) = self.kafka.send_receipt(&receipt_json).await {
            tracing::warn!("Failed to send receipt to Kafka: {}", e);
        }

        // Archive to S3 (non-blocking, best effort)
        if let Err(e) = self.s3.archive_receipt(&receipt_json).await {
            tracing::warn!("Failed to archive receipt to S3: {}", e);
        }

        Ok(receipt)
    }
}

