# Quick Start Guide

## 🚀 Get Running in 5 Minutes

### Step 1: Start Infrastructure
```bash
cd infrastructure
docker-compose up -d postgres zookeeper kafka opa
```

Wait ~30 seconds for services to be ready.

### Step 2: Build & Run Services

**Option A: Run Locally (Recommended for Development)**

```bash
# Terminal 1: Identity Registry
cd services/identity-registry
DATABASE_URL="postgresql://postgres:postgres@localhost:5433/pathwell" PORT=3001 cargo run

# Terminal 2: Policy Engine Wrapper  
cd services/policy-engine
OPA_URL="http://localhost:8181" PORT=3002 cargo run

# Terminal 3: Receipt Store
cd services/receipt-store
KAFKA_BROKERS="localhost:9092" KAFKA_TOPIC="pathwell-receipts" \
S3_BUCKET="pathwell-receipts" S3_REGION="us-east-1" \
DATABASE_URL="postgresql://postgres:postgres@localhost:5433/pathwell" \
PORT=3003 cargo run

# Terminal 4: Proxy Gateway
cd services/proxy-gateway
TARGET_BACKEND_URL="http://httpbin.org" \
IDENTITY_REGISTRY_URL="http://localhost:3001" \
POLICY_ENGINE_URL="http://localhost:3002" \
RECEIPT_STORE_URL="http://localhost:3003" \
PORT=8080 cargo run
```

**Option B: Docker Compose (All Services)**
```bash
cd infrastructure
docker-compose up --build
```

### Step 3: Register Developer & Agent

```bash
# Generate keys (Python)
python3 << EOF
from pathwell import generate_key_pair
priv, pub = generate_key_pair()
with open('/tmp/agent.key', 'w') as f:
    f.write(priv)
print("Public key:")
print(pub)
EOF

# Register developer
PUB_KEY=$(python3 -c "from pathwell import generate_key_pair; print(generate_key_pair()[1])")
curl -X POST http://localhost:3001/v1/developers/register \
  -H "Content-Type: application/json" \
  -d "{\"developer_id\": \"test-dev\", \"public_key\": \"$PUB_KEY\"}"

# Register agent
curl -X POST http://localhost:3001/v1/agents/register \
  -H "Content-Type: application/json" \
  -d "{\"agent_id\": \"test-agent\", \"developer_id\": \"test-dev\", \"public_key\": \"$PUB_KEY\"}"
```

### Step 4: Test the Flow

```python
from pathwell import PathwellClient

client = PathwellClient(
    agent_id="test-agent",
    private_key_path="/tmp/agent.key",
    proxy_url="http://localhost:8080"
)

# Make a request through proxy
response = client.get("http://httpbin.org/get")
print(f"Status: {response.status_code}")
print(f"Response: {response.text[:200]}")
```

### Step 5: Verify Receipts

```bash
# Check Kafka topic (if kafkacat installed)
kafkacat -b localhost:9092 -t pathwell-receipts -C -e

# Or check logs
docker-compose logs receipt-store | grep "Receipt"
```

## ✅ Success Indicators

- All services return `OK` on `/health` endpoints
- Agent registration succeeds
- Requests through proxy return 200
- Receipts appear in Kafka/S3

## 🐛 Troubleshooting

**Services won't start:**
- Check Docker is running: `docker ps`
- Check ports aren't in use: `lsof -i :3001,3002,3003,8080`
- Check logs: `docker-compose logs <service>`

**Database errors:**
- Wait for PostgreSQL to be ready: `docker-compose logs postgres | grep "ready"`
- Check connection string matches docker-compose.yml

**Kafka errors:**
- Wait for Zookeeper: `docker-compose logs zookeeper`
- Check Kafka is healthy: `docker-compose ps kafka`

**OPA errors:**
- Check policies are mounted: `docker-compose exec opa ls /policies`
- Check OPA health: `curl http://localhost:8181/health`

## 📚 Next Steps

See [NEXT_STEPS.md](NEXT_STEPS.md) for:
- Production deployment
- Adding features
- Performance optimization
- Security hardening

