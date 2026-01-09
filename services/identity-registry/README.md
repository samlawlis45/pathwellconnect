# Identity Registry Service

The Anchor - Identity Registry for Pathwell Connect MVG.

## Overview

The Identity Registry is responsible for:
- Registering agents and issuing cryptographic credentials
- Validating agent identities
- Managing developer and enterprise relationships
- Revoking agent credentials

## Technology Stack

- Rust with Tokio async runtime
- PostgreSQL for credential storage
- PKI (Public Key Infrastructure) for certificate generation
- Axum web framework

## API Endpoints

### Register Agent
```
POST /v1/agents/register
Body: {
  "agent_id": "string",
  "developer_id": "string",
  "enterprise_id": "string (optional)",
  "public_key": "PEM-encoded public key"
}
```

### Validate Agent
```
GET /v1/agents/{agent_id}/validate
Response: {
  "valid": boolean,
  "agent_id": "string",
  "developer_id": "uuid",
  "enterprise_id": "uuid (optional)",
  "revoked": boolean
}
```

### Revoke Agent
```
POST /v1/agents/{agent_id}/revoke
Body: {
  "reason": "string (optional)"
}
```

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string (default: `postgresql://postgres:postgres@localhost:5432/pathwell`)
- `PORT`: Server port (default: `3001`)

## Running

```bash
# Set environment variables
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/pathwell"
export PORT=3001

# Run migrations and start server
cargo run
```

## Database Schema

See `migrations/001_initial_schema.sql` for the complete schema.

