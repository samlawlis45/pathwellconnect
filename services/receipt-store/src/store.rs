use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::receipt::{Receipt, ReceiptRequest, EventSource, ExternalEvent, ExternalEventRequest};
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
}

