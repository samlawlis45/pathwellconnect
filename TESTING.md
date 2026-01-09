# Testing Pathwell Connect MVG

## Current Status

✅ **Identity Registry**: Running on port 3001

## Next Steps: Test the Flow

### 1. Register a Developer

```bash
# Generate a key pair first (using Python SDK)
python3 << 'EOF'
import sys
sys.path.insert(0, 'sdks/python')
from pathwell import generate_key_pair

private_key, public_key = generate_key_pair()
with open('/tmp/test_agent.key', 'w') as f:
    f.write(private_key)
print("Public key:")
print(public_key)
EOF

# Save the public key, then register developer
PUB_KEY="<paste-public-key-here>"

curl -X POST http://localhost:3001/v1/developers/register \
  -H "Content-Type: application/json" \
  -d "{
    \"developer_id\": \"test-developer-001\",
    \"public_key\": \"$PUB_KEY\"
  }"
```

### 2. Register an Agent

```bash
curl -X POST http://localhost:3001/v1/agents/register \
  -H "Content-Type: application/json" \
  -d "{
    \"agent_id\": \"test-agent-001\",
    \"developer_id\": \"test-developer-001\",
    \"public_key\": \"$PUB_KEY\"
  }"
```

### 3. Validate the Agent

```bash
curl http://localhost:3001/v1/agents/test-agent-001/validate
```

### 4. Test with Python SDK

```python
from pathwell import PathwellClient

client = PathwellClient(
    agent_id="test-agent-001",
    private_key_path="/tmp/test_agent.key",
    proxy_url="http://localhost:8080"  # Will work once proxy is running
)

# This will work once Proxy Gateway is running
# response = client.get("http://httpbin.org/get")
# print(response.json())
```

## Building Other Services

### Policy Engine Wrapper

```bash
cd services/policy-engine
OPA_URL="http://localhost:8181" PORT=3002 cargo run
```

### Receipt Store

```bash
cd services/receipt-store
# First, run migrations
docker exec -i infrastructure-postgres-1 psql -U postgres -d pathwell < migrations/001_receipts_table.sql

# Then run the service
KAFKA_BROKERS="localhost:9092" KAFKA_TOPIC="pathwell-receipts" \
S3_BUCKET="pathwell-receipts" S3_REGION="us-east-1" \
DATABASE_URL="postgresql://postgres:postgres@localhost:5433/pathwell" \
PORT=3003 cargo run
```

### Proxy Gateway

```bash
cd services/proxy-gateway
TARGET_BACKEND_URL="http://httpbin.org" \
IDENTITY_REGISTRY_URL="http://localhost:3001" \
POLICY_ENGINE_URL="http://localhost:3002" \
RECEIPT_STORE_URL="http://localhost:3003" \
PORT=8080 cargo run
```

## Quick Test Script

Save this as `test-flow.sh`:

```bash
#!/bin/bash

# 1. Generate keys
python3 << 'PYTHON'
import sys
sys.path.insert(0, 'sdks/python')
from pathwell import generate_key_pair

priv, pub = generate_key_pair()
with open('/tmp/test_agent.key', 'w') as f:
    f.write(priv)
print(pub)
PYTHON

# 2. Register developer
PUB_KEY=$(python3 -c "import sys; sys.path.insert(0, 'sdks/python'); from pathwell import generate_key_pair; print(generate_key_pair()[1])")

curl -X POST http://localhost:3001/v1/developers/register \
  -H "Content-Type: application/json" \
  -d "{\"developer_id\": \"test-dev\", \"public_key\": \"$PUB_KEY\"}"

# 3. Register agent  
curl -X POST http://localhost:3001/v1/agents/register \
  -H "Content-Type: application/json" \
  -d "{\"agent_id\": \"test-agent\", \"developer_id\": \"test-dev\", \"public_key\": \"$PUB_KEY\"}"

# 4. Validate
curl http://localhost:3001/v1/agents/test-agent/validate
```

