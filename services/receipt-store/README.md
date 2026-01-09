# Receipt Store Service

The Proof - Receipt Store for Pathwell Connect MVG.

## Overview

The Receipt Store creates immutable, hash-chained receipts for every decision made by the system. Receipts are stored in:
1. **Kafka**: Real-time streaming for immediate access
2. **S3**: Long-term archival with partitioning
3. **PostgreSQL** (optional): Hash lookup table for chain verification

## Technology Stack

- Rust with Tokio async runtime
- Apache Kafka for streaming
- AWS S3 for archival
- PostgreSQL (optional) for hash chain verification
- Axum web framework

## Receipt Format

```json
{
  "receipt_id": "uuid",
  "timestamp": "iso8601",
  "agent_id": "string",
  "request": {
    "method": "POST",
    "path": "/api/v1/chat",
    "headers": {},
    "body_hash": "sha256"
  },
  "policy_result": {
    "allowed": true,
    "policy_version": "v1",
    "evaluation_time_ms": 5
  },
  "identity_result": {
    "valid": true,
    "developer_id": "uuid",
    "enterprise_id": "uuid"
  },
  "receipt_hash": "sha256",
  "previous_receipt_hash": "sha256"
}
```

## Hash Chaining

Each receipt includes the hash of the previous receipt, creating an immutable chain. This allows verification that no receipts have been tampered with or removed.

## API Endpoints

### Store Receipt
```
POST /v1/receipts
Body: {
  "agent_id": "string",
  "request": {...},
  "policy_result": {...},
  "identity_result": {...}
}

Response: {
  "receipt_id": "uuid",
  "receipt_hash": "sha256",
  "stored": true
}
```

## Environment Variables

- `KAFKA_BROKERS`: Kafka broker addresses (default: `localhost:9092`)
- `KAFKA_TOPIC`: Kafka topic name (default: `pathwell-receipts`)
- `S3_BUCKET`: S3 bucket name (default: `pathwell-receipts`)
- `S3_REGION`: AWS region (default: `us-east-1`)
- `DATABASE_URL`: PostgreSQL connection string (optional)
- `PORT`: Server port (default: `3003`)

## Running

```bash
export KAFKA_BROKERS="localhost:9092"
export KAFKA_TOPIC="pathwell-receipts"
export S3_BUCKET="pathwell-receipts"
export S3_REGION="us-east-1"
export PORT=3003

cargo run
```

## S3 Partitioning

Receipts are stored in S3 with the following partition structure:
```
s3://bucket/receipts/YYYY/MM/DD/HH/receipt_timestamp.json
```

This enables efficient querying by time range.

