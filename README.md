# Pathwell Connect MVG

Minimum Viable Governance (MVG) for Pathwell Connect - A control plane for AI agent governance.

## Overview

Pathwell Connect MVG implements a fail-closed governance system that intercepts all agent calls, validates identity, evaluates policies, and creates immutable receipts. The system follows the principle: **No Identity, No Run**.

## Architecture

The MVP consists of four core pillars:

1. **The Anchor (Identity Registry)**: PostgreSQL + PKI for agent credential management
2. **The Intercept (Proxy Gateway)**: Rust reverse proxy that intercepts all requests
3. **The Constitution (Policy Engine)**: OPA-based policy evaluation engine
4. **The Proof (Receipt Store)**: Kafka + S3 for immutable receipt storage

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Rust (for building services)
- Python 3.8+ (for SDK and tests)

### Start Services

```bash
cd infrastructure
cp .env.example .env
# Edit .env and set TARGET_BACKEND_URL
docker-compose up -d
```

### Register an Agent

```python
from pathwell import PathwellClient, generate_key_pair

# Generate keys
private_key, public_key = generate_key_pair()

# Save private key
with open("agent.key", "w") as f:
    f.write(private_key)

# Register agent (via Identity Registry API)
import requests
requests.post("http://localhost:3001/v1/agents/register", json={
    "agent_id": "my-agent",
    "developer_id": "my-developer",
    "public_key": public_key
})
```

### Use the SDK

```python
from pathwell import PathwellClient

client = PathwellClient(
    agent_id="my-agent",
    private_key_path="./agent.key",
    proxy_url="http://localhost:8080"
)

# Make requests through proxy
response = client.post(
    url="https://api.example.com/v1/chat",
    body={"message": "Hello"}
)
```

## Project Structure

```
PathwellConnect/
├── services/
│   ├── identity-registry/    # Identity Registry service
│   ├── proxy-gateway/         # Proxy Gateway service
│   ├── policy-engine/         # Policy Engine service
│   └── receipt-store/         # Receipt Store service
├── sdks/
│   ├── python/                # Python SDK
│   ├── typescript/            # TypeScript SDK
│   └── go/                    # Go SDK
├── infrastructure/
│   └── docker-compose.yml     # Docker Compose setup
└── tests/
    ├── integration_test.py   # Integration tests
    └── load_test.py           # Load tests
```

## Services

### Identity Registry (Port 3001)

Manages agent credentials and PKI certificates.

**Endpoints:**
- `POST /v1/agents/register` - Register agent
- `GET /v1/agents/{agent_id}/validate` - Validate agent
- `POST /v1/agents/{agent_id}/revoke` - Revoke agent

### Policy Engine (Port 3002)

Evaluates requests against policies using OPA.

**Endpoints:**
- `POST /v1/evaluate` - Evaluate policy

### Receipt Store (Port 3003)

Stores immutable receipts in Kafka and S3.

**Endpoints:**
- `POST /v1/receipts` - Store receipt

### Proxy Gateway (Port 8080)

Main entry point that intercepts and governs all requests.

## SDKs

SDKs are available for:
- **Python**: `sdks/python/`
- **TypeScript/JavaScript**: `sdks/typescript/`
- **Go**: `sdks/go/`

## Testing

Run integration tests:
```bash
python tests/integration_test.py
```

Run load tests:
```bash
python tests/load_test.py
```

## Design Principles

- **Fail-Closed**: Default deny, explicit allow
- **Immutable Receipts**: Hash-chained for tamper detection
- **No Identity, No Run**: All requests require valid agent identity
- **Policy as Code**: Policies are code (Rego), not PDFs

## License

MIT

