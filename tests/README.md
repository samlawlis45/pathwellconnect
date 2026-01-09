# Pathwell Connect MVG Tests

Integration and load tests for Pathwell Connect MVG.

## Prerequisites

1. All services running (via Docker Compose)
2. Python 3.8+
3. Test dependencies installed:
```bash
pip install requests
```

## Running Tests

### Integration Tests

Tests the end-to-end flow:
- Service health checks
- Agent registration
- Proxy request flow
- Receipt generation

```bash
python tests/integration_test.py
```

### Load Tests

Tests latency targets (<100ms for policy evaluation + forwarding):

```bash
python tests/load_test.py
```

Environment variables:
- `CONCURRENT_REQUESTS`: Number of concurrent requests (default: 10)
- `TOTAL_REQUESTS`: Total number of requests (default: 100)
- `PROXY_URL`: Proxy URL (default: http://localhost:8080)
- `TARGET_BACKEND_URL`: Target backend URL (default: http://httpbin.org)

Example:
```bash
CONCURRENT_REQUESTS=20 TOTAL_REQUESTS=500 python tests/load_test.py
```

## Test Coverage

- ✓ Service health checks
- ✓ Agent registration and validation
- ✓ Proxy request forwarding
- ✓ Policy evaluation
- ✓ Receipt generation
- ✓ Latency measurements
- ⚠ Policy denial (requires revoked agent setup)

