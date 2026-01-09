# Pathwell Connect Infrastructure

Docker Compose setup for running all Pathwell Connect MVG services.

## Services

- **PostgreSQL**: Database for Identity Registry and Receipt Store
- **Zookeeper + Kafka**: Message streaming for receipts
- **OPA**: Policy Engine server
- **Identity Registry**: Agent credential management (port 3001)
- **Policy Engine**: Policy evaluation wrapper (port 3002)
- **Receipt Store**: Receipt storage service (port 3003)
- **Proxy Gateway**: Main proxy service (port 8080)

## Quick Start

1. Copy environment file:
```bash
cp .env.example .env
```

2. Edit `.env` and set `TARGET_BACKEND_URL` to your target backend

3. Start all services:
```bash
docker-compose up -d
```

4. Check service health:
```bash
docker-compose ps
```

## Building Services

To rebuild services:
```bash
docker-compose build
```

## Stopping Services

```bash
docker-compose down
```

To remove volumes:
```bash
docker-compose down -v
```

## Health Checks

All services expose health check endpoints:
- Identity Registry: `http://localhost:3001/health`
- Policy Engine: `http://localhost:3002/health`
- Receipt Store: `http://localhost:3003/health`
- Proxy Gateway: `http://localhost:8080/health`

## Environment Variables

See `.env.example` for required environment variables.

## Kafka Topics

The receipt store creates a topic `pathwell-receipts` automatically.

## Database Migrations

Migrations run automatically on service startup for:
- Identity Registry
- Receipt Store

