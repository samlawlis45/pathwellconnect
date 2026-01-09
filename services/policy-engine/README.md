# Policy Engine Service

The Constitution - Policy Engine for Pathwell Connect MVG.

## Overview

The Policy Engine evaluates requests against policies using OPA (Open Policy Agent). It implements a pluggable architecture to support multiple policy engines in the future (e.g., Cedar).

## Technology Stack

- Rust with Tokio async runtime
- OPA (Open Policy Agent) for policy evaluation
- Axum web framework
- Pluggable engine architecture

## Architecture

The service consists of:
1. **OPA Server**: Docker container running OPA with Rego policies
2. **Policy Engine Wrapper**: Rust service that provides REST API and interfaces with OPA
3. **Pluggable Engine Trait**: Abstract interface for supporting multiple policy engines

## API Endpoints

### Evaluate Policy
```
POST /v1/evaluate
Body: {
  "agent": {
    "valid": boolean,
    "revoked": boolean,
    "agent_id": "string",
    "developer_id": "string",
    "enterprise_id": "string (optional)"
  },
  "request": {
    "method": "string",
    "path": "string",
    "headers": {},
    "body_hash": "string (optional)"
  }
}

Response: {
  "allowed": boolean,
  "reason": "string",
  "evaluation_time_ms": number
}
```

## Policy Format

Policies are written in Rego and stored in `policies/pathwell.rego`. The default policy:
- Default deny (fail-closed)
- Requires agent to be valid and not revoked
- Checks request method and path against allowed patterns

## Environment Variables

- `OPA_URL`: OPA server URL (default: `http://localhost:8181`)
- `PORT`: Server port (default: `3002`)

## Running

### Start OPA Server
```bash
docker build -t pathwell-opa .
docker run -p 8181:8181 pathwell-opa
```

### Start Policy Engine Wrapper
```bash
export OPA_URL="http://localhost:8181"
export PORT=3002
cargo run
```

## Policy Development

Edit `policies/pathwell.rego` to modify policies. The policy uses Rego v1 syntax and supports:
- Agent validation checks
- Request method/path matching
- Custom business logic rules

