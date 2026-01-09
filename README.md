# Pathwell Connect

A governance platform for AI agent transactions with an **Intelligent Ledger** - a transaction lineage explorer that tracks every checkpoint, decision, and actor interaction across your enterprise systems.

## Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Proxy Gateway  │────▶│ Identity Registry│     │  Policy Engine  │
│    (Rust)       │     │     (Rust)       │     │   (OPA/Rego)    │
│   Port 8080     │     │   Port 3001      │     │   Port 8181     │
└────────┬────────┘     └──────────────────┘     └─────────────────┘
         │
         ▼
┌─────────────────┐     ┌──────────────────┐
│  Receipt Store  │────▶│    Dashboard     │
│    (Rust)       │     │   (Next.js)      │
│   Port 3003     │     │   Port 3000      │
└────────┬────────┘     └──────────────────┘
         │
    ┌────┴────┐
    ▼         ▼
┌────────┐ ┌───────┐
│Postgres│ │ Kafka │
│ :5433  │ │ :9092 │
└────────┘ └───────┘
```

## Services

| Service | Port | Description |
|---------|------|-------------|
| **Proxy Gateway** | 8080 | Intercepts API calls, extracts correlation IDs, enforces policies |
| **Identity Registry** | 3001 | Manages agent identities and credentials |
| **Policy Engine** | 8181 | OPA-based policy evaluation |
| **Receipt Store** | 3003 | Immutable transaction ledger with trace queries |
| **Dashboard** | 3000 | Next.js UI for exploring transaction traces |
| **PostgreSQL** | 5433 | Primary database |
| **Kafka** | 9092 | Event streaming |

## Prerequisites

- Docker & Docker Compose
- Rust 1.85+ (for local development)
- Node.js 18+ (for dashboard development)

## Quick Start

### Option 1: Docker Compose (Full Stack)

```bash
cd infrastructure
docker-compose up -d
```

Wait for all services to be healthy, then open http://localhost:3000

### Option 2: Local Development

1. **Start infrastructure services:**

```bash
cd infrastructure
docker-compose up -d postgres kafka zookeeper opa
```

2. **Run database migrations:**

```bash
cd services/receipt-store
PGPASSWORD=postgres psql -h localhost -p 5433 -U postgres -d pathwell -f migrations/001_initial_schema.sql
PGPASSWORD=postgres psql -h localhost -p 5433 -U postgres -d pathwell -f migrations/002_intelligent_ledger.sql
```

3. **Start the Receipt Store:**

```bash
cd services/receipt-store
DATABASE_URL="postgresql://postgres:postgres@localhost:5433/pathwell" \
KAFKA_BROKERS="localhost:9092" \
cargo run
```

4. **Start the Dashboard:**

```bash
cd dashboard
npm install
NEXT_PUBLIC_API_URL=http://localhost:3003 npm run dev
```

Open http://localhost:3000

## Intelligent Ledger

The Intelligent Ledger is a "flight tracker for enterprise transactions" - enter a reference number and see every checkpoint, decision, and actor interaction.

### Dashboard Features

- **Dashboard** (`/`) - Overview stats and recent traces
- **Traces** (`/traces`) - Browse and filter all transaction traces
- **Lookup** (`/lookup`) - Search by correlation ID (flight-tracker style)
- **Trace Detail** (`/traces/:id`) - Timeline view, decision tree, raw data

### Key Concepts

- **Trace**: A group of related events sharing a `trace_id`
- **Correlation ID**: External reference (e.g., `PO-2024-001`) linking to your business systems
- **Span**: Individual event within a trace
- **Decision Tree**: Visual representation of policy evaluation flow

## API Reference

### Receipt Store (Port 3003)

#### Write Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/receipts` | Store a transaction receipt |
| `POST` | `/v1/events/external` | Ingest external system events |

#### Read Endpoints (Intelligent Ledger)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/v1/traces` | List traces with filtering |
| `GET` | `/v1/traces/:trace_id` | Get trace details |
| `GET` | `/v1/traces/:trace_id/timeline` | Get chronological event timeline |
| `GET` | `/v1/traces/:trace_id/decisions` | Get decision tree structure |
| `GET` | `/v1/lookup/:correlation_id` | Lookup trace by external reference |

### Query Parameters for `/v1/traces`

| Parameter | Type | Description |
|-----------|------|-------------|
| `correlation_id` | string | Filter by external reference |
| `agent_id` | string | Filter by agent |
| `status` | string | Filter by status (active, completed, failed) |
| `limit` | number | Results per page (default: 20) |
| `offset` | number | Pagination offset |

### Example: Store a Receipt

```bash
curl -X POST http://localhost:3003/v1/receipts \
  -H "Content-Type: application/json" \
  -d '{
    "agent_id": "sales-agent-001",
    "enterprise_id": "acme-corp",
    "action": "create_order",
    "resource": "/api/orders",
    "outcome": "allowed",
    "correlation_id": "PO-2024-001",
    "event_type": "api_request",
    "event_source": "proxy_gateway"
  }'
```

### Example: Ingest External Event

```bash
curl -X POST http://localhost:3003/v1/events/external \
  -H "Content-Type: application/json" \
  -d '{
    "correlation_id": "PO-2024-001",
    "source_system": "SAP",
    "event_type": "order_confirmed",
    "summary": "Order confirmed in SAP ERP",
    "details": {"sap_order_id": "4500012345"},
    "outcome": {"success": true}
  }'
```

### Example: Query Traces

```bash
# List all traces
curl http://localhost:3003/v1/traces

# Filter by correlation ID
curl "http://localhost:3003/v1/traces?correlation_id=PO-2024-001"

# Lookup by correlation ID
curl http://localhost:3003/v1/lookup/PO-2024-001

# Get timeline for a trace
curl http://localhost:3003/v1/traces/{trace_id}/timeline
```

## Other Services

### Identity Registry (Port 3001)

Manages agent credentials and PKI certificates.

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/v1/agents/register` | Register agent |
| `GET` | `/v1/agents/:agent_id/validate` | Validate agent |
| `POST` | `/v1/agents/:agent_id/revoke` | Revoke agent |

### Proxy Gateway (Port 8080)

Main entry point that intercepts and governs all requests. Extracts `x-correlation-id` headers for trace linking.

## Project Structure

```
PathwellConnect/
├── services/
│   ├── proxy-gateway/      # Rust - API interception & routing
│   ├── identity-registry/  # Rust - Agent identity management
│   ├── policy-engine/      # OPA/Rego - Policy definitions
│   └── receipt-store/      # Rust - Transaction ledger & queries
├── dashboard/              # Next.js - Intelligent Ledger UI
├── sdks/
│   ├── python/             # Python SDK
│   ├── typescript/         # TypeScript SDK
│   └── go/                 # Go SDK
├── infrastructure/         # Docker Compose configuration
└── README.md
```

## Environment Variables

### Receipt Store

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | - | PostgreSQL connection string |
| `KAFKA_BROKERS` | localhost:9092 | Kafka broker addresses |
| `KAFKA_TOPIC` | pathwell-receipts | Topic for receipt events |
| `PORT` | 3003 | HTTP server port |

### Dashboard

| Variable | Default | Description |
|----------|---------|-------------|
| `NEXT_PUBLIC_API_URL` | http://localhost:3003 | Receipt Store API URL |

### Proxy Gateway

| Variable | Default | Description |
|----------|---------|-------------|
| `TARGET_BACKEND_URL` | http://httpbin.org | Backend to proxy requests to |
| `IDENTITY_REGISTRY_URL` | - | Identity Registry service URL |
| `POLICY_ENGINE_URL` | - | Policy Engine service URL |
| `RECEIPT_STORE_URL` | - | Receipt Store service URL |

## Design Principles

- **Fail-Closed**: Default deny, explicit allow
- **Immutable Receipts**: Hash-chained for tamper detection
- **No Identity, No Run**: All requests require valid agent identity
- **Policy as Code**: Policies are code (Rego), not PDFs
- **Full Lineage**: Every transaction traceable end-to-end

## License

MIT
