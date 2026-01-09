# Pathwell Connect TypeScript SDK

TypeScript/JavaScript SDK for Pathwell Connect MVG.

## Installation

```bash
npm install
npm run build
```

## Usage

```typescript
import { PathwellClient } from '@pathwell/connect';

// Initialize client
const client = new PathwellClient({
  agentId: 'agent-123',
  privateKeyPath: './agent.key',
  proxyUrl: 'https://proxy.pathwell.io'
});

// Make requests
const response = await client.post(
  'https://api.example.com/v1/chat',
  { message: 'Hello' }
);

const data = await response.json();
console.log(data);
```

## Generating Keys

```typescript
import { generateKeyPair } from '@pathwell/connect';
import * as fs from 'fs';

const { privateKey, publicKey } = generateKeyPair();

// Save keys
fs.writeFileSync('agent.key', privateKey);
fs.writeFileSync('agent.pub', publicKey);
```

## API Reference

### PathwellClient

Main client class for making authenticated requests.

#### Constructor

```typescript
new PathwellClient(options: PathwellClientOptions)
```

#### Methods

- `call(method, url, options)`: Make a request
- `get(url, options)`: GET request
- `post(url, body, options)`: POST request
- `put(url, body, options)`: PUT request
- `patch(url, body, options)`: PATCH request
- `delete(url, options)`: DELETE request

