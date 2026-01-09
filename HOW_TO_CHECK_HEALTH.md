# How to Check if OPA (and other services) are Healthy

## Quick Commands

### Check OPA Health Status

**Method 1: Docker Compose Status**
```bash
cd infrastructure
docker-compose ps opa
```
Look for `(healthy)` in the STATUS column.

**Method 2: HTTP Health Endpoint**
```bash
curl http://localhost:8181/health
```
If it returns `{}` (empty JSON), OPA is healthy.

**Method 3: Check Logs**
```bash
cd infrastructure
docker-compose logs opa | tail -10
```
Look for:
- ✅ `"msg":"Initializing server."` - Starting up
- ✅ No `error:` messages - Healthy
- ❌ `error: initialization error` - Policy has errors

### Check All Services

**Quick Status Check:**
```bash
cd infrastructure
docker-compose ps
```

**Use the Health Check Script:**
```bash
./check-health.sh
```

## Service Health Indicators

### PostgreSQL ✅
- Status shows `(healthy)`
- Port 5433 accessible
- Test: `psql -h localhost -p 5433 -U postgres -d pathwell -c "SELECT 1;"`

### Zookeeper ✅  
- Status shows `(healthy)`
- Port 2181 accessible

### Kafka ⚠️
- Container is `Up`
- May take 30-60 seconds to fully start
- Check logs: `docker-compose logs kafka`

### OPA ✅
- Status shows `(healthy)` 
- `curl http://localhost:8181/health` returns `{}`
- No errors in logs

## Troubleshooting OPA

**If OPA shows "unhealthy":**

1. Check for policy errors:
   ```bash
   docker-compose logs opa | grep error
   ```

2. Test the policy manually:
   ```bash
   docker-compose exec opa opa test /policies
   ```

3. Check if OPA is actually running:
   ```bash
   docker-compose exec opa opa version
   ```

4. Restart OPA:
   ```bash
   docker-compose restart opa
   ```

## Expected Startup Times

- PostgreSQL: ~10 seconds
- Zookeeper: ~15 seconds  
- Kafka: ~30-60 seconds
- OPA: ~5-10 seconds (if policy is valid)

## When Services are Ready

All services are ready when:
- `docker-compose ps` shows all services as `(healthy)` or `Up`
- Health endpoints respond:
  - PostgreSQL: Can connect via psql
  - OPA: `curl http://localhost:8181/health` returns `{}`
  - Kafka: Logs show "started"

Then you can start building and running the Rust services!

