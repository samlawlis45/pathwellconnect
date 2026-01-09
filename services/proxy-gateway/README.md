# Proxy Gateway Service

The Intercept - Proxy Gateway for Pathwell Connect MVG.

## Overview

The Proxy Gateway is the choke point for all agent requests. It:
1. Intercepts incoming requests
2. Validates agent identity
3. Evaluates policies
4. Forwards allowed requests or denies blocked requests
5. Generates immutable receipts for all decisions

## Technology Stack

- Rust with Tokio async runtime
- Axum web framework
- Reqwest for HTTP forwarding
- Target: <100ms latency for policy evaluation + forwarding

## Request Flow

1. **Ingress**: Agent calls SDK with `pathwell.call()`
2. **Halt**: Proxy intercepts the request
3. **Adjudicate**: 
   - Validate agent identity via Identity Registry
   - Evaluate request against Policy Engine
4. **Execute**:
   - If valid: Forward to target infrastructure
   - If invalid: Return 403 with reason
5. **Witness**: Generate receipt and send to Receipt Store (async)

## Required Headers

- `X-Pathwell-Agent-ID`: Agent identifier
- `X-Pathwell-Signature`: Request signature (for future use)

## Environment Variables

- `TARGET_BACKEND_URL`: Target backend URL (required)
- `IDENTITY_REGISTRY_URL`: Identity Registry service URL (default: `http://localhost:3001`)
- `POLICY_ENGINE_URL`: Policy Engine service URL (default: `http://localhost:3002`)
- `RECEIPT_STORE_URL`: Receipt Store service URL (default: `http://localhost:3003`)
- `PORT`: Listen port (default: `8080`)
- `LISTEN_HOST`: Listen host (default: `0.0.0.0`)

## Running

```bash
export TARGET_BACKEND_URL="https://api.example.com"
export IDENTITY_REGISTRY_URL="http://localhost:3001"
export POLICY_ENGINE_URL="http://localhost:3002"
export RECEIPT_STORE_URL="http://localhost:3003"
export PORT=8080

cargo run
```

## Fail-Closed Design

The proxy follows a fail-closed governance model:
- Default deny: If identity validation fails, request is denied
- Default deny: If policy evaluation fails, request is denied
- Default deny: If policy is silent, request is denied

All decisions (allow or deny) are recorded as receipts.

