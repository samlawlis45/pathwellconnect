# Running Pathwell Connect Services

## Navigation Note

**Important:** Run these commands from the **project root** (`/Users/samlawlis/PathwellConnect`), not from the `infrastructure` directory.

## Quick Commands

### Identity Registry
```bash
cd /Users/samlawlis/PathwellConnect/services/identity-registry
DATABASE_URL="postgresql://postgres:postgres@localhost:5433/pathwell" PORT=3001 cargo run
```

### Policy Engine Wrapper
```bash
cd /Users/samlawlis/PathwellConnect/services/policy-engine
OPA_URL="http://localhost:8181" PORT=3002 cargo run
```

### Receipt Store
```bash
cd /Users/samlawlis/PathwellConnect/services/receipt-store
KAFKA_BROKERS="localhost:9092" KAFKA_TOPIC="pathwell-receipts" \
S3_BUCKET="pathwell-receipts" S3_REGION="us-east-1" \
DATABASE_URL="postgresql://postgres:postgres@localhost:5433/pathwell" \
PORT=3003 cargo run
```

### Proxy Gateway
```bash
cd /Users/samlawlis/PathwellConnect/services/proxy-gateway
TARGET_BACKEND_URL="http://httpbin.org" \
IDENTITY_REGISTRY_URL="http://localhost:3001" \
POLICY_ENGINE_URL="http://localhost:3002" \
RECEIPT_STORE_URL="http://localhost:3003" \
PORT=8080 cargo run
```

## First Time Setup

If you haven't built the services yet, you may need to:

1. **Install Rust** (if not installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Check compilation** (downloads dependencies):
   ```bash
   cd services/identity-registry
   cargo check
   ```

3. **Fix any compilation errors** before running

## Running All Services

Open 4 terminal windows/tabs, one for each service, and run them in this order:

1. Identity Registry (port 3001)
2. Policy Engine (port 3002)  
3. Receipt Store (port 3003)
4. Proxy Gateway (port 8080)

## Verify Services

Once running, test each service:
```bash
curl http://localhost:3001/health  # Identity Registry
curl http://localhost:3002/health  # Policy Engine
curl http://localhost:3003/health  # Receipt Store
curl http://localhost:8080/health  # Proxy Gateway
```

