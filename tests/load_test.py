#!/usr/bin/env python3
"""
Load tests for Pathwell Connect MVG
Tests latency targets (<100ms for policy evaluation + forwarding)
"""

import os
import sys
import time
import statistics
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "sdks" / "python"))

from pathwell import PathwellClient


class LoadTestConfig:
    """Load test configuration"""
    PROXY_URL = os.getenv("PROXY_URL", "http://localhost:8080")
    TARGET_BACKEND_URL = os.getenv("TARGET_BACKEND_URL", "http://httpbin.org")
    CONCURRENT_REQUESTS = int(os.getenv("CONCURRENT_REQUESTS", "10"))
    TOTAL_REQUESTS = int(os.getenv("TOTAL_REQUESTS", "100"))
    AGENT_ID = os.getenv("TEST_AGENT_ID", "test-agent-001")
    PRIVATE_KEY_PATH = os.getenv("PRIVATE_KEY_PATH", "/tmp/test_agent.key")


def make_request(client: PathwellClient, request_num: int):
    """Make a single request and return latency"""
    start = time.time()
    try:
        resp = client.get(f"{LoadTestConfig.TARGET_BACKEND_URL}/get")
        latency = (time.time() - start) * 1000  # Convert to ms
        return {
            "request_num": request_num,
            "latency_ms": latency,
            "status_code": resp.status_code,
            "success": resp.status_code == 200,
        }
    except Exception as e:
        latency = (time.time() - start) * 1000
        return {
            "request_num": request_num,
            "latency_ms": latency,
            "status_code": 0,
            "success": False,
            "error": str(e),
        }


def run_load_test():
    """Run load test"""
    print("=" * 60)
    print("Pathwell Connect MVG Load Test")
    print("=" * 60)
    print(f"Concurrent requests: {LoadTestConfig.CONCURRENT_REQUESTS}")
    print(f"Total requests: {LoadTestConfig.TOTAL_REQUESTS}")
    print()
    
    # Initialize client
    client = PathwellClient(
        agent_id=LoadTestConfig.AGENT_ID,
        private_key_path=LoadTestConfig.PRIVATE_KEY_PATH,
        proxy_url=LoadTestConfig.PROXY_URL
    )
    
    results = []
    start_time = time.time()
    
    # Run requests with concurrency
    with ThreadPoolExecutor(max_workers=LoadTestConfig.CONCURRENT_REQUESTS) as executor:
        futures = [
            executor.submit(make_request, client, i)
            for i in range(LoadTestConfig.TOTAL_REQUESTS)
        ]
        
        for future in as_completed(futures):
            result = future.result()
            results.append(result)
            if result["success"]:
                print(f"Request {result['request_num']}: {result['latency_ms']:.2f}ms")
            else:
                print(f"Request {result['request_num']}: FAILED - {result.get('error', 'Unknown error')}")
    
    total_time = time.time() - start_time
    
    # Calculate statistics
    latencies = [r["latency_ms"] for r in results if r["success"]]
    successes = sum(1 for r in results if r["success"])
    failures = len(results) - successes
    
    if latencies:
        avg_latency = statistics.mean(latencies)
        median_latency = statistics.median(latencies)
        p95_latency = statistics.quantiles(latencies, n=20)[18] if len(latencies) > 1 else latencies[0]
        p99_latency = statistics.quantiles(latencies, n=100)[98] if len(latencies) > 1 else latencies[0]
        min_latency = min(latencies)
        max_latency = max(latencies)
    else:
        avg_latency = median_latency = p95_latency = p99_latency = min_latency = max_latency = 0
    
    # Print results
    print()
    print("=" * 60)
    print("Load Test Results")
    print("=" * 60)
    print(f"Total requests: {LoadTestConfig.TOTAL_REQUESTS}")
    print(f"Successful: {successes}")
    print(f"Failed: {failures}")
    print(f"Success rate: {(successes/len(results)*100):.2f}%")
    print(f"Total time: {total_time:.2f}s")
    print(f"Requests/sec: {len(results)/total_time:.2f}")
    print()
    print("Latency Statistics:")
    print(f"  Average: {avg_latency:.2f}ms")
    print(f"  Median: {median_latency:.2f}ms")
    print(f"  P95: {p95_latency:.2f}ms")
    print(f"  P99: {p99_latency:.2f}ms")
    print(f"  Min: {min_latency:.2f}ms")
    print(f"  Max: {max_latency:.2f}ms")
    print()
    
    # Check latency target
    target_ms = 100
    if avg_latency > target_ms:
        print(f"⚠ WARNING: Average latency ({avg_latency:.2f}ms) exceeds target ({target_ms}ms)")
    else:
        print(f"✓ Average latency ({avg_latency:.2f}ms) meets target ({target_ms}ms)")
    
    if p95_latency > target_ms * 2:
        print(f"⚠ WARNING: P95 latency ({p95_latency:.2f}ms) exceeds 2x target ({target_ms*2}ms)")
    
    print("=" * 60)
    
    return 0 if avg_latency <= target_ms * 2 else 1


if __name__ == "__main__":
    sys.exit(run_load_test())

