# Next Steps: Getting Pathwell Connect MVG Running

## Immediate Actions

### 1. Fix Missing Dependencies & Build Issues

The code is written but needs to be compiled and tested. Here's what to check:

#### Rust Services
```bash
# Test each service compiles
cd services/identity-registry && cargo check
cd ../policy-engine && cargo check
cd ../receipt-store && cargo check
cd ../proxy-gateway && cargo check
```

**Potential Issues to Fix:**
- Missing `Cargo.lock` files (will be generated on first build)
- SQLx compile-time checks may fail without database - use `sqlx prepare` or disable checks
- Missing dependencies in Cargo.toml

#### Python SDK
```bash
cd sdks/python
pip install -e .
python -c "from pathwell import PathwellClient; print('OK')"
```

#### TypeScript SDK
```bash
cd sdks/typescript
npm install
npm run build
```

#### Go SDK
```bash
cd sdks/go
go mod tidy
go build ./...
```

### 2. Set Up Development Environment

#### Option A: Local Development (Recommended for Testing)

1. **Start Infrastructure Services:**
```bash
cd infrastructure
docker-compose up -d postgres zookeeper kafka opa
```

2. **Run Rust Services Locally:**
```bash
# Terminal 1: Identity Registry
cd services/identity-registry
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/pathwell"
cargo run

# Terminal 2: Policy Engine Wrapper
cd services/policy-engine
export OPA_URL="http://localhost:8181"
export PORT=3002
cargo run

# Terminal 3: Receipt Store
cd services/receipt-store
export KAFKA_BROKERS="localhost:9092"
export KAFKA_TOPIC="pathwell-receipts"
export S3_BUCKET="pathwell-receipts"  # Or use local MinIO
export PORT=3003
cargo run

# Terminal 4: Proxy Gateway
cd services/proxy-gateway
export TARGET_BACKEND_URL="http://httpbin.org"
export IDENTITY_REGISTRY_URL="http://localhost:3001"
export POLICY_ENGINE_URL="http://localhost:3002"
export RECEIPT_STORE_URL="http://localhost:3003"
export PORT=8080
cargo run
```

#### Option B: Full Docker Compose (Production-like)

```bash
cd infrastructure
# Build all services first
docker-compose build

# Start everything
docker-compose up -d

# Check logs
docker-compose logs -f
```

### 3. Fix Known Issues

#### Database Migrations
- Identity Registry uses `sqlx::migrate!` macro - ensure migrations directory is accessible
- May need to adjust path in `main.rs` or use `sqlx migrate run` manually

#### S3 Configuration
- Receipt Store requires AWS credentials or MinIO for local development
- Consider adding MinIO to docker-compose.yml for local testing

#### OPA Policy Loading
- Verify OPA Dockerfile correctly mounts policies directory
- Check OPA can read `pathwell.rego` file

### 4. Test the System

#### Step 1: Health Checks
```bash
curl http://localhost:3001/health  # Identity Registry
curl http://localhost:3002/health  # Policy Engine
curl http://localhost:3003/health  # Receipt Store
curl http://localhost:8080/health  # Proxy Gateway
```

#### Step 2: Register a Developer & Agent
```bash
# First, register a developer (if endpoint exists, or skip for MVP)
# Then register an agent
curl -X POST http://localhost:3001/v1/agents/register \
  -H "Content-Type: application/json" \
  -d '{
    "agent_id": "test-agent-001",
    "developer_id": "test-dev-001",
    "public_key": "<generated-public-key>"
  }'
```

#### Step 3: Test End-to-End Flow
```bash
# Use Python SDK
cd tests
python integration_test.py
```

### 5. Critical Fixes Needed

#### A. Developer Registration
The Identity Registry expects developers to exist, but there's no registration endpoint. Add:
```rust
// In services/identity-registry/src/api/handlers.rs
pub async fn register_developer(...) { ... }
```

#### B. Request Signing
SDKs sign requests, but Proxy Gateway doesn't verify signatures yet. Either:
- Implement signature verification in Proxy Gateway
- Or remove signature requirement for MVP (just validate agent_id)

#### C. Error Handling
- Add proper error responses
- Add request validation
- Add rate limiting (optional for MVP)

#### D. Configuration
- Add configuration files (TOML/YAML) instead of just env vars
- Add logging configuration
- Add metrics/observability (optional)

### 6. Documentation Updates

- [ ] Add API documentation (OpenAPI/Swagger)
- [ ] Add deployment guide
- [ ] Add troubleshooting guide
- [ ] Add architecture diagrams
- [ ] Add example policies

### 7. Testing & Validation

#### Unit Tests
```bash
# Add unit tests for each service
cd services/identity-registry
cargo test

# Repeat for other services
```

#### Integration Tests
```bash
# Fix integration tests to work with actual services
cd tests
python integration_test.py
```

#### Load Tests
```bash
# Verify <100ms latency target
python tests/load_test.py
```

### 8. Production Readiness Checklist

- [ ] Add TLS/HTTPS support
- [ ] Add authentication for service-to-service communication
- [ ] Add monitoring/alerting (Prometheus/Grafana)
- [ ] Add distributed tracing (Jaeger/Zipkin)
- [ ] Add rate limiting
- [ ] Add request/response validation
- [ ] Add comprehensive error handling
- [ ] Add security headers
- [ ] Add CORS configuration
- [ ] Add database connection pooling
- [ ] Add retry logic for external services
- [ ] Add circuit breakers
- [ ] Add graceful shutdown

### 9. Next Features (Post-MVG)

- [ ] Context Graph / Precedent Search
- [ ] Hardware Anchor Check (TPM/HSM integration)
- [ ] Settlement Layer
- [ ] Marketplace
- [ ] Multi-protocol support (gRPC, WebSocket)
- [ ] Policy versioning
- [ ] Policy rollback
- [ ] Receipt query API
- [ ] Receipt verification API

## Quick Start Checklist

- [ ] Install Rust, Docker, Python
- [ ] Clone repository
- [ ] Build Rust services (`cargo build`)
- [ ] Start infrastructure (`docker-compose up -d`)
- [ ] Run services locally or via Docker
- [ ] Register a developer (or add endpoint)
- [ ] Register an agent
- [ ] Test with Python SDK
- [ ] Run integration tests
- [ ] Verify receipts are generated

## Getting Help

If you encounter issues:

1. Check service logs: `docker-compose logs <service-name>`
2. Verify environment variables are set
3. Check database connectivity
4. Verify Kafka is running
5. Check OPA is serving policies
6. Review error messages in service logs

## Recommended Development Flow

1. **Week 1**: Fix build issues, get all services running locally
2. **Week 2**: Fix integration issues, get end-to-end flow working
3. **Week 3**: Add missing features (developer registration, signature verification)
4. **Week 4**: Add tests, documentation, production hardening

