use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Query parameters for trace listing
#[derive(Debug, Deserialize)]
pub struct TraceQuery {
    pub correlation_id: Option<String>,
    pub agent_id: Option<String>,
    pub enterprise_id: Option<Uuid>,
    pub status: Option<String>,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Trace summary for list view
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TraceSummary {
    pub trace_id: Uuid,
    pub correlation_id: Option<String>,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub last_event_at: DateTime<Utc>,
    pub event_count: i32,
    pub policy_deny_count: i32,
    pub initiating_agent_id: Option<String>,
    pub initiating_developer_id: Option<Uuid>,
    pub initiating_enterprise_id: Option<Uuid>,
}

/// Response for trace list
#[derive(Debug, Serialize)]
pub struct TraceListResponse {
    pub traces: Vec<TraceSummary>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// Timeline event for visualization
#[derive(Debug, Serialize)]
pub struct TimelineEvent {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub source_system: String,
    pub source_service: String,
    pub agent_id: Option<String>,
    pub summary: String,
    pub outcome: EventOutcome,
    pub details: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct EventOutcome {
    pub success: bool,
    pub reason: Option<String>,
}

/// Decision tree for visualization
#[derive(Debug, Serialize)]
pub struct DecisionTree {
    pub nodes: Vec<DecisionNode>,
    pub edges: Vec<DecisionEdge>,
}

#[derive(Debug, Serialize)]
pub struct DecisionNode {
    pub id: String,
    pub node_type: String,
    pub label: String,
    pub outcome: bool,
    pub timestamp: DateTime<Utc>,
    pub details: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct DecisionEdge {
    pub from: String,
    pub to: String,
    pub label: Option<String>,
}

/// Full trace detail response
#[derive(Debug, Serialize)]
pub struct TraceDetailResponse {
    pub trace: TraceSummary,
    pub timeline: Vec<TimelineEvent>,
    pub decision_tree: DecisionTree,
}

/// Raw receipt event from database
#[derive(Debug, sqlx::FromRow)]
pub struct ReceiptEventRow {
    pub id: Uuid,
    pub receipt_id: Uuid,
    pub trace_id: Uuid,
    pub correlation_id: Option<String>,
    pub span_id: Uuid,
    pub parent_span_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub event_source_system: String,
    pub event_source_service: String,
    pub event_source_version: Option<String>,
    pub agent_id: Option<String>,
    pub developer_id: Option<Uuid>,
    pub enterprise_id: Option<Uuid>,
    pub request_method: Option<String>,
    pub request_path: Option<String>,
    pub request_headers: Option<serde_json::Value>,
    pub request_body_hash: Option<String>,
    pub policy_allowed: Option<bool>,
    pub policy_version: Option<String>,
    pub policy_evaluation_ms: Option<i32>,
    pub identity_valid: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub full_receipt: serde_json::Value,
    pub receipt_hash: String,
    pub previous_receipt_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Raw external event from database
#[derive(Debug, sqlx::FromRow)]
pub struct ExternalEventRow {
    pub id: Uuid,
    pub event_id: Uuid,
    pub trace_id: Uuid,
    pub correlation_id: Option<String>,
    pub event_type: String,
    pub source_system: String,
    pub source_id: String,
    pub timestamp: DateTime<Utc>,
    pub actor_type: Option<String>,
    pub actor_id: Option<String>,
    pub actor_display_name: Option<String>,
    pub payload: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

pub struct QueryService {
    pool: PgPool,
}

impl QueryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// List traces with filtering and pagination
    pub async fn list_traces(&self, params: TraceQuery) -> Result<TraceListResponse> {
        let limit = params.limit.unwrap_or(50).min(100);
        let offset = params.offset.unwrap_or(0);

        // Build dynamic query
        let traces: Vec<TraceSummary> = sqlx::query_as(
            r#"
            SELECT trace_id, correlation_id, status, started_at, last_event_at,
                   event_count, policy_deny_count, initiating_agent_id,
                   initiating_developer_id, initiating_enterprise_id
            FROM traces
            WHERE ($1::text IS NULL OR correlation_id = $1)
              AND ($2::text IS NULL OR initiating_agent_id = $2)
              AND ($3::uuid IS NULL OR initiating_enterprise_id = $3)
              AND ($4::text IS NULL OR status = $4)
              AND ($5::timestamptz IS NULL OR started_at >= $5)
              AND ($6::timestamptz IS NULL OR started_at <= $6)
            ORDER BY last_event_at DESC
            LIMIT $7 OFFSET $8
            "#
        )
        .bind(&params.correlation_id)
        .bind(&params.agent_id)
        .bind(&params.enterprise_id)
        .bind(&params.status)
        .bind(&params.from)
        .bind(&params.to)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        // Get total count
        let (total,): (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM traces
            WHERE ($1::text IS NULL OR correlation_id = $1)
              AND ($2::text IS NULL OR initiating_agent_id = $2)
              AND ($3::uuid IS NULL OR initiating_enterprise_id = $3)
              AND ($4::text IS NULL OR status = $4)
              AND ($5::timestamptz IS NULL OR started_at >= $5)
              AND ($6::timestamptz IS NULL OR started_at <= $6)
            "#
        )
        .bind(&params.correlation_id)
        .bind(&params.agent_id)
        .bind(&params.enterprise_id)
        .bind(&params.status)
        .bind(&params.from)
        .bind(&params.to)
        .fetch_one(&self.pool)
        .await?;

        Ok(TraceListResponse {
            traces,
            total,
            limit,
            offset,
        })
    }

    /// Get a single trace by ID
    pub async fn get_trace(&self, trace_id: Uuid) -> Result<Option<TraceSummary>> {
        let trace: Option<TraceSummary> = sqlx::query_as(
            r#"
            SELECT trace_id, correlation_id, status, started_at, last_event_at,
                   event_count, policy_deny_count, initiating_agent_id,
                   initiating_developer_id, initiating_enterprise_id
            FROM traces
            WHERE trace_id = $1
            "#
        )
        .bind(trace_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(trace)
    }

    /// Get trace by correlation ID
    pub async fn get_trace_by_correlation(&self, correlation_id: &str) -> Result<Option<TraceSummary>> {
        let trace: Option<TraceSummary> = sqlx::query_as(
            r#"
            SELECT trace_id, correlation_id, status, started_at, last_event_at,
                   event_count, policy_deny_count, initiating_agent_id,
                   initiating_developer_id, initiating_enterprise_id
            FROM traces
            WHERE correlation_id = $1
            "#
        )
        .bind(correlation_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(trace)
    }

    /// Get receipt events for a trace
    pub async fn get_receipt_events(&self, trace_id: Uuid) -> Result<Vec<ReceiptEventRow>> {
        let events: Vec<ReceiptEventRow> = sqlx::query_as(
            r#"
            SELECT id, receipt_id, trace_id, correlation_id, span_id, parent_span_id,
                   timestamp, event_type, event_source_system, event_source_service, event_source_version,
                   agent_id, developer_id, enterprise_id,
                   request_method, request_path, request_headers, request_body_hash,
                   policy_allowed, policy_version, policy_evaluation_ms, identity_valid,
                   metadata, full_receipt, receipt_hash, previous_receipt_hash, created_at
            FROM receipt_events
            WHERE trace_id = $1
            ORDER BY timestamp ASC
            "#
        )
        .bind(trace_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(events)
    }

    /// Get external events for a trace
    pub async fn get_external_events(&self, trace_id: Uuid) -> Result<Vec<ExternalEventRow>> {
        let events: Vec<ExternalEventRow> = sqlx::query_as(
            r#"
            SELECT id, event_id, trace_id, correlation_id, event_type, source_system, source_id,
                   timestamp, actor_type, actor_id, actor_display_name, payload, metadata, created_at
            FROM external_events
            WHERE trace_id = $1
            ORDER BY timestamp ASC
            "#
        )
        .bind(trace_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(events)
    }

    /// Build timeline from all events
    pub async fn get_timeline(&self, trace_id: Uuid) -> Result<Vec<TimelineEvent>> {
        let receipt_events = self.get_receipt_events(trace_id).await?;
        let external_events = self.get_external_events(trace_id).await?;

        let mut timeline: Vec<TimelineEvent> = Vec::new();

        // Convert receipt events to timeline events
        for event in receipt_events {
            let summary = format!(
                "{} {} - {}",
                event.request_method.as_deref().unwrap_or("?"),
                event.request_path.as_deref().unwrap_or("?"),
                if event.policy_allowed.unwrap_or(false) { "Allowed" } else { "Denied" }
            );

            timeline.push(TimelineEvent {
                event_id: event.receipt_id,
                timestamp: event.timestamp,
                event_type: event.event_type.clone(),
                source_system: event.event_source_system,
                source_service: event.event_source_service,
                agent_id: event.agent_id,
                summary,
                outcome: EventOutcome {
                    success: event.policy_allowed.unwrap_or(false) && event.identity_valid.unwrap_or(false),
                    reason: if !event.policy_allowed.unwrap_or(true) {
                        Some("Policy denied".to_string())
                    } else if !event.identity_valid.unwrap_or(true) {
                        Some("Identity invalid".to_string())
                    } else {
                        None
                    },
                },
                details: event.full_receipt,
            });
        }

        // Convert external events to timeline events
        for event in external_events {
            let actor_name = event.actor_display_name
                .or(event.actor_id.clone())
                .unwrap_or_else(|| "System".to_string());

            timeline.push(TimelineEvent {
                event_id: event.event_id,
                timestamp: event.timestamp,
                event_type: event.event_type.clone(),
                source_system: event.source_system.clone(),
                source_service: event.source_id,
                agent_id: event.actor_id,
                summary: format!("{} by {} ({})", event.event_type, actor_name, event.source_system),
                outcome: EventOutcome {
                    success: true,
                    reason: None,
                },
                details: event.payload,
            });
        }

        // Sort by timestamp
        timeline.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(timeline)
    }

    /// Build decision tree from receipt events
    pub async fn build_decision_tree(&self, trace_id: Uuid) -> Result<DecisionTree> {
        let events = self.get_receipt_events(trace_id).await?;

        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for (i, event) in events.iter().enumerate() {
            let identity_valid = event.identity_valid.unwrap_or(false);
            let policy_allowed = event.policy_allowed.unwrap_or(false);

            // Identity node
            let identity_node_id = format!("identity-{}", i);
            nodes.push(DecisionNode {
                id: identity_node_id.clone(),
                node_type: "identity".to_string(),
                label: format!("Identity: {}", event.agent_id.as_deref().unwrap_or("unknown")),
                outcome: identity_valid,
                timestamp: event.timestamp,
                details: serde_json::json!({
                    "agent_id": event.agent_id,
                    "valid": identity_valid,
                    "developer_id": event.developer_id,
                    "enterprise_id": event.enterprise_id,
                }),
            });

            // Policy node
            let policy_node_id = format!("policy-{}", i);
            nodes.push(DecisionNode {
                id: policy_node_id.clone(),
                node_type: "policy".to_string(),
                label: format!("Policy: {}", event.policy_version.as_deref().unwrap_or("v1")),
                outcome: policy_allowed,
                timestamp: event.timestamp,
                details: serde_json::json!({
                    "allowed": policy_allowed,
                    "version": event.policy_version,
                    "evaluation_ms": event.policy_evaluation_ms,
                }),
            });

            // Action node (the actual request)
            let action_node_id = format!("action-{}", i);
            nodes.push(DecisionNode {
                id: action_node_id.clone(),
                node_type: "action".to_string(),
                label: format!(
                    "{} {}",
                    event.request_method.as_deref().unwrap_or("?"),
                    event.request_path.as_deref().unwrap_or("?")
                ),
                outcome: identity_valid && policy_allowed,
                timestamp: event.timestamp,
                details: serde_json::json!({
                    "method": event.request_method,
                    "path": event.request_path,
                    "body_hash": event.request_body_hash,
                }),
            });

            // Edges
            edges.push(DecisionEdge {
                from: identity_node_id.clone(),
                to: policy_node_id.clone(),
                label: Some(if identity_valid { "valid" } else { "invalid" }.to_string()),
            });

            edges.push(DecisionEdge {
                from: policy_node_id.clone(),
                to: action_node_id.clone(),
                label: Some(if policy_allowed { "allowed" } else { "denied" }.to_string()),
            });

            // Connect to previous action if exists
            if i > 0 {
                edges.push(DecisionEdge {
                    from: format!("action-{}", i - 1),
                    to: identity_node_id,
                    label: Some("next".to_string()),
                });
            }
        }

        Ok(DecisionTree { nodes, edges })
    }

    /// Get full trace detail with timeline and decision tree
    pub async fn get_trace_detail(&self, trace_id: Uuid) -> Result<Option<TraceDetailResponse>> {
        let trace = match self.get_trace(trace_id).await? {
            Some(t) => t,
            None => return Ok(None),
        };

        let timeline = self.get_timeline(trace_id).await?;
        let decision_tree = self.build_decision_tree(trace_id).await?;

        Ok(Some(TraceDetailResponse {
            trace,
            timeline,
            decision_tree,
        }))
    }
}
