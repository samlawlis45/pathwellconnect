#!/bin/bash
# Quick health check script for Pathwell Connect services

echo "🔍 Checking Pathwell Connect Infrastructure Health..."
echo ""

# Check PostgreSQL
echo -n "PostgreSQL (5433): "
if docker ps --filter "name=infrastructure-postgres" --format "{{.Status}}" | grep -q "healthy"; then
    echo "✅ Healthy"
else
    echo "❌ Not healthy"
fi

# Check Zookeeper
echo -n "Zookeeper (2181): "
if docker ps --filter "name=infrastructure-zookeeper" --format "{{.Status}}" | grep -q "healthy"; then
    echo "✅ Healthy"
else
    echo "❌ Not healthy"
fi

# Check Kafka
echo -n "Kafka (9092): "
if docker ps --filter "name=infrastructure-kafka" --format "{{.Status}}" | grep -q "Up"; then
    echo "✅ Running"
else
    echo "❌ Not running"
fi

# Check OPA
echo -n "OPA (8181): "
if curl -s http://localhost:8181/health > /dev/null 2>&1; then
    echo "✅ Healthy"
elif docker ps --filter "name=infrastructure-opa" --format "{{.Status}}" | grep -q "Up"; then
    echo "⏳ Starting (check logs: docker-compose logs opa)"
else
    echo "❌ Not running"
fi

echo ""
echo "📊 Full status:"
cd "$(dirname "$0")/infrastructure" && docker-compose ps 2>/dev/null || echo "Run from project root: cd infrastructure && docker-compose ps"

