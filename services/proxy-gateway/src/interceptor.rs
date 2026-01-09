use anyhow::Result;
use hyper::{Request, Response, StatusCode};
use hyper::header::HeaderValue;
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use hex;
use reqwest;

use crate::identity_client::IdentityClient;
use crate::policy_client::PolicyClient;
use crate::receipt_client::{
    ReceiptClient, ReceiptRequest, RequestInfo as ReceiptRequestInfo,
    PolicyResult, IdentityResult, EventType, EventSource
};
use crate::config::Config;
use uuid::Uuid;

const AGENT_ID_HEADER: &str = "x-pathwell-agent-id";
const SIGNATURE_HEADER: &str = "x-pathwell-signature";
const CORRELATION_ID_HEADER: &str = "x-correlation-id";
const TRACE_ID_HEADER: &str = "x-pathwell-trace-id";

pub struct Interceptor {
    config: Config,
    identity_client: IdentityClient,
    policy_client: PolicyClient,
    receipt_client: ReceiptClient,
}

/// Trace context extracted from or generated for a request
struct TraceContext {
    trace_id: Uuid,
    correlation_id: Option<String>,
    span_id: Uuid,
}

impl Interceptor {
    pub fn new(config: Config) -> Self {
        Self {
            identity_client: IdentityClient::new(config.identity_registry_url.clone()),
            policy_client: PolicyClient::new(config.policy_engine_url.clone()),
            receipt_client: ReceiptClient::new(config.receipt_store_url.clone()),
            config,
        }
    }

    /// Extract trace context from headers or generate new one
    fn extract_trace_context(headers: &HashMap<String, String>) -> TraceContext {
        // Try to get existing trace ID from header, or generate new one
        let trace_id = headers
            .get(TRACE_ID_HEADER)
            .or_else(|| headers.get(&TRACE_ID_HEADER.to_lowercase()))
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);

        // Extract correlation ID if present (external reference number)
        let correlation_id = headers
            .get(CORRELATION_ID_HEADER)
            .or_else(|| headers.get(&CORRELATION_ID_HEADER.to_lowercase()))
            .cloned();

        // Always generate a new span ID for this request
        let span_id = Uuid::new_v4();

        TraceContext {
            trace_id,
            correlation_id,
            span_id,
        }
    }

    pub async fn intercept(
        &self,
        mut parts: http::request::Parts,
        body_bytes: hyper::body::Bytes,
    ) -> Result<Response<hyper::body::Bytes>> {
        let start_time = std::time::Instant::now();

        // Extract agent ID from headers before moving parts
        let agent_id = parts.headers
            .get(AGENT_ID_HEADER)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing {} header", AGENT_ID_HEADER))?;

        let body_hash = Some(hex::encode(Sha256::digest(&body_bytes)));

        // Reconstruct request with body for extracting details
        let req = Request::from_parts(parts, hyper::body::Bytes::from(body_bytes.clone()));

        // Extract request details
        let method = req.method().to_string();
        let path = req.uri().path().to_string();
        let headers: HashMap<String, String> = req.headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        // Extract or generate trace context
        let trace_ctx = Self::extract_trace_context(&headers);

        // Step 1: Validate identity
        let identity_result = match self.identity_client.validate_agent(&agent_id).await {
            Ok(result) => {
                if !result.valid || result.revoked {
                    return self.create_error_response(
                        StatusCode::FORBIDDEN,
                        "Agent identity invalid or revoked",
                        &agent_id,
                        &trace_ctx,
                        method,
                        path,
                        headers,
                        body_hash,
                        start_time,
                    ).await;
                }
                result
            }
            Err(e) => {
                tracing::error!("Identity validation failed: {}", e);
                return self.create_error_response(
                    StatusCode::FORBIDDEN,
                    &format!("Identity validation failed: {}", e),
                    &agent_id,
                    &trace_ctx,
                    method,
                    path,
                    headers,
                    body_hash,
                    start_time,
                ).await;
            }
        };

        // Step 2: Evaluate policy
        let policy_result = match self.policy_client.evaluate(
            &agent_id,
            identity_result.valid,
            identity_result.revoked,
            identity_result.developer_id,
            identity_result.enterprise_id,
            &method,
            &path,
            &headers,
            body_hash.clone(),
        ).await {
            Ok(result) => {
                if !result.allowed {
                    return self.create_error_response(
                        StatusCode::FORBIDDEN,
                        &result.reason,
                        &agent_id,
                        &trace_ctx,
                        method,
                        path,
                        headers,
                        body_hash,
                        start_time,
                    ).await;
                }
                result
            }
            Err(e) => {
                tracing::error!("Policy evaluation failed: {}", e);
                // Fail closed - deny on policy engine error
                return self.create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Policy evaluation failed: {}", e),
                    &agent_id,
                    &trace_ctx,
                    method,
                    path,
                    headers,
                    body_hash,
                    start_time,
                ).await;
            }
        };

        // Step 3: Forward request to target backend
        let target_uri = format!("{}{}", self.config.target_backend_url, path);

        // Use reqwest for forwarding
        let client = reqwest::Client::new();
        let mut target_req = match method.as_str() {
            "GET" => client.get(&target_uri),
            "POST" => client.post(&target_uri),
            "PUT" => client.put(&target_uri),
            "PATCH" => client.patch(&target_uri),
            "DELETE" => client.delete(&target_uri),
            _ => {
                return self.create_error_response(
                    StatusCode::METHOD_NOT_ALLOWED,
                    "Unsupported HTTP method",
                    &agent_id,
                    &trace_ctx,
                    method,
                    path,
                    headers,
                    body_hash,
                    start_time,
                ).await;
            }
        };

        // Copy headers (except Pathwell headers)
        for (key, value) in &headers {
            if !key.to_lowercase().starts_with("x-pathwell-") &&
               key.to_lowercase() != "host" &&
               key.to_lowercase() != "content-length" {
                target_req = target_req.header(key, value);
            }
        }

        // Propagate trace context to downstream services
        target_req = target_req.header(TRACE_ID_HEADER, trace_ctx.trace_id.to_string());
        if let Some(ref corr_id) = trace_ctx.correlation_id {
            target_req = target_req.header(CORRELATION_ID_HEADER, corr_id);
        }

        // Add body if present
        if !body_bytes.is_empty() {
            target_req = target_req.body(body_bytes.clone());
        }

        let response = match target_req.send().await {
            Ok(resp) => resp,
            Err(e) => {
                tracing::error!("Failed to forward request: {}", e);
                return self.create_error_response(
                    StatusCode::BAD_GATEWAY,
                    &format!("Failed to forward request: {}", e),
                    &agent_id,
                    &trace_ctx,
                    method,
                    path,
                    headers,
                    body_hash,
                    start_time,
                ).await;
            }
        };

        // Convert reqwest response to hyper response
        let status = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let mut hyper_response = Response::builder().status(status);

        // Copy response headers
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                hyper_response = hyper_response.header(key.as_str(), value_str);
            }
        }

        // Add trace ID to response for client tracking
        hyper_response = hyper_response.header(TRACE_ID_HEADER, trace_ctx.trace_id.to_string());

        let body = response.bytes().await.unwrap_or_default();
        let hyper_response = hyper_response.body(hyper::body::Bytes::from(body))?;

        // Step 4: Generate receipt (async, non-blocking)
        let receipt = ReceiptRequest {
            trace_id: trace_ctx.trace_id,
            correlation_id: trace_ctx.correlation_id.clone(),
            span_id: trace_ctx.span_id,
            parent_span_id: None,
            agent_id: agent_id.to_string(),
            event_type: EventType::GatewayRequest,
            event_source: EventSource::default(),
            request: ReceiptRequestInfo {
                method: method.clone(),
                path: path.clone(),
                headers: headers.clone(),
                body_hash: body_hash.clone(),
            },
            policy_result: PolicyResult {
                allowed: policy_result.allowed,
                policy_version: "v1".to_string(),
                evaluation_time_ms: policy_result.evaluation_time_ms,
            },
            identity_result: IdentityResult {
                valid: identity_result.valid,
                developer_id: identity_result.developer_id,
                enterprise_id: identity_result.enterprise_id,
            },
            metadata: None,
        };

        // Store receipt asynchronously
        let _ = self.receipt_client.store_receipt(receipt).await;

        Ok(hyper_response)
    }

    async fn create_error_response(
        &self,
        status: StatusCode,
        reason: &str,
        agent_id: &str,
        trace_ctx: &TraceContext,
        method: String,
        path: String,
        headers: HashMap<String, String>,
        body_hash: Option<String>,
        start_time: std::time::Instant,
    ) -> Result<Response<hyper::body::Bytes>> {
        // Generate receipt for denied request
        let receipt = ReceiptRequest {
            trace_id: trace_ctx.trace_id,
            correlation_id: trace_ctx.correlation_id.clone(),
            span_id: trace_ctx.span_id,
            parent_span_id: None,
            agent_id: agent_id.to_string(),
            event_type: EventType::GatewayRequest,
            event_source: EventSource::default(),
            request: ReceiptRequestInfo {
                method,
                path,
                headers,
                body_hash,
            },
            policy_result: PolicyResult {
                allowed: false,
                policy_version: "v1".to_string(),
                evaluation_time_ms: start_time.elapsed().as_millis() as u64,
            },
            identity_result: IdentityResult {
                valid: false,
                developer_id: Uuid::nil(),
                enterprise_id: None,
            },
            metadata: Some(serde_json::json!({
                "error_reason": reason,
                "status_code": status.as_u16(),
            })),
        };

        // Store receipt asynchronously
        let _ = self.receipt_client.store_receipt(receipt).await;

        let body = serde_json::json!({
            "error": "request_denied",
            "reason": reason,
            "status": status.as_u16(),
            "trace_id": trace_ctx.trace_id.to_string(),
        });

        let response = Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .header(TRACE_ID_HEADER, trace_ctx.trace_id.to_string())
            .body(hyper::body::Bytes::from(serde_json::to_string(&body)?))?;

        Ok(response)
    }
}
