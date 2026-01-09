# Pathwell Connect Python SDK

Python SDK for Pathwell Connect MVG.

## Installation

```bash
pip install -e .
```

## Usage

```python
from pathwell import PathwellClient

# Initialize client
client = PathwellClient(
    agent_id="agent-123",
    private_key_path="./agent.key",
    proxy_url="https://proxy.pathwell.io"
)

# Make requests
response = client.post(
    url="https://api.example.com/v1/chat",
    body={"message": "Hello"}
)

print(response.json())
```

## Generating Keys

```python
from pathwell import generate_key_pair

private_key, public_key = generate_key_pair()

# Save keys
with open("agent.key", "w") as f:
    f.write(private_key)

with open("agent.pub", "w") as f:
    f.write(public_key)
```

## API Reference

### PathwellClient

Main client class for making authenticated requests.

#### Methods

- `call(method, url, headers=None, body=None, **kwargs)`: Make a request
- `get(url, **kwargs)`: GET request
- `post(url, body=None, **kwargs)`: POST request
- `put(url, body=None, **kwargs)`: PUT request
- `patch(url, body=None, **kwargs)`: PATCH request
- `delete(url, **kwargs)`: DELETE request

