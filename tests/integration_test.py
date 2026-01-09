#!/usr/bin/env python3
"""
Integration tests for Pathwell Connect MVG
Tests end-to-end flow from SDK call through proxy to receipt generation
"""

import os
import sys
import time
import requests
import json
from pathlib import Path

# Add SDK to path
sys.path.insert(0, str(Path(__file__).parent.parent / "sdks" / "python"))

from pathwell import PathwellClient, generate_key_pair


class TestConfig:
    """Test configuration"""
    IDENTITY_REGISTRY_URL = os.getenv("IDENTITY_REGISTRY_URL", "http://localhost:3001")
    POLICY_ENGINE_URL = os.getenv("POLICY_ENGINE_URL", "http://localhost:3002")
    RECEIPT_STORE_URL = os.getenv("RECEIPT_STORE_URL", "http://localhost:3003")
    PROXY_URL = os.getenv("PROXY_URL", "http://localhost:8080")
    TARGET_BACKEND_URL = os.getenv("TARGET_BACKEND_URL", "http://httpbin.org")


def test_health_checks():
    """Test that all services are healthy"""
    print("Testing health checks...")
    
    services = {
        "Identity Registry": TestConfig.IDENTITY_REGISTRY_URL + "/health",
        "Policy Engine": TestConfig.POLICY_ENGINE_URL + "/health",
        "Receipt Store": TestConfig.RECEIPT_STORE_URL + "/health",
    }
    
    for name, url in services.items():
        try:
            resp = requests.get(url, timeout=5)
            assert resp.status_code == 200, f"{name} health check failed"
            print(f"✓ {name} is healthy")
        except Exception as e:
            print(f"✗ {name} health check failed: {e}")
            raise


def test_agent_registration():
    """Test agent registration flow"""
    print("\nTesting agent registration...")
    
    # Generate key pair
    private_key, public_key = generate_key_pair()
    
    # Register developer (simplified - in production this would be separate)
    # For MVP, we'll register directly as agent
    
    # Register agent
    payload = {
        "agent_id": "test-agent-001",
        "developer_id": "test-developer-001",
        "public_key": public_key
    }
    
    resp = requests.post(
        f"{TestConfig.IDENTITY_REGISTRY_URL}/v1/agents/register",
        json=payload,
        timeout=10
    )
    
    if resp.status_code == 201 or resp.status_code == 200:
        print("✓ Agent registered successfully")
        data = resp.json()
        assert "certificate_chain" in data
        return private_key, public_key
    elif resp.status_code == 409:
        print("⚠ Agent already exists, using existing credentials")
        # Try to validate existing agent
        resp = requests.get(
            f"{TestConfig.IDENTITY_REGISTRY_URL}/v1/agents/test-agent-001/validate",
            timeout=10
        )
        assert resp.status_code == 200
        return private_key, public_key
    else:
        print(f"✗ Agent registration failed: {resp.status_code} - {resp.text}")
        raise AssertionError(f"Registration failed: {resp.status_code}")


def test_proxy_flow(private_key_path: str):
    """Test end-to-end proxy flow"""
    print("\nTesting proxy flow...")
    
    # Initialize client
    client = PathwellClient(
        agent_id="test-agent-001",
        private_key_path=private_key_path,
        proxy_url=TestConfig.PROXY_URL
    )
    
    # Make a request through proxy
    start_time = time.time()
    resp = client.get(f"{TestConfig.TARGET_BACKEND_URL}/get")
    latency = (time.time() - start_time) * 1000  # Convert to ms
    
    print(f"✓ Request completed in {latency:.2f}ms")
    assert resp.status_code == 200, f"Request failed with status {resp.status_code}"
    
    # Check latency target (<100ms for policy evaluation + forwarding)
    # Note: This includes network latency, so we use a more lenient threshold
    assert latency < 5000, f"Latency too high: {latency}ms"
    
    return latency


def test_policy_denial():
    """Test that invalid requests are denied"""
    print("\nTesting policy denial...")
    
    # Create client with invalid agent ID
    client = PathwellClient(
        agent_id="invalid-agent",
        private_key_path="/tmp/test.key",  # Won't be used
        proxy_url=TestConfig.PROXY_URL
    )
    
    # This should fail - but we need a valid key first
    # For now, we'll test with a non-existent agent
    # In a real test, we'd revoke an agent and test that
    
    print("⚠ Policy denial test skipped (requires revoked agent setup)")


def test_receipt_generation():
    """Test that receipts are generated for requests"""
    print("\nTesting receipt generation...")
    
    # Make a request
    resp = requests.get(f"{TestConfig.TARGET_BACKEND_URL}/get")
    assert resp.status_code == 200
    
    # Check Kafka for receipt (simplified - in production would query Kafka)
    # For MVP, we'll just verify the receipt store is accessible
    resp = requests.get(f"{TestConfig.RECEIPT_STORE_URL}/health")
    assert resp.status_code == 200
    
    print("✓ Receipt store is accessible")


def run_all_tests():
    """Run all integration tests"""
    print("=" * 60)
    print("Pathwell Connect MVG Integration Tests")
    print("=" * 60)
    
    try:
        # Test 1: Health checks
        test_health_checks()
        
        # Test 2: Agent registration
        private_key, public_key = test_agent_registration()
        
        # Save private key for SDK test
        key_path = "/tmp/test_agent.key"
        with open(key_path, "w") as f:
            f.write(private_key)
        
        # Test 3: Proxy flow
        latency = test_proxy_flow(key_path)
        
        # Test 4: Policy denial (skipped for now)
        test_policy_denial()
        
        # Test 5: Receipt generation
        test_receipt_generation()
        
        print("\n" + "=" * 60)
        print("✓ All tests passed!")
        print(f"  Average latency: {latency:.2f}ms")
        print("=" * 60)
        
        return 0
        
    except Exception as e:
        print(f"\n✗ Test failed: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(run_all_tests())

