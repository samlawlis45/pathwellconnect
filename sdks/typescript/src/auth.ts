import * as crypto from 'crypto';
import * as fs from 'fs';

export interface KeyPair {
  privateKey: string;
  publicKey: string;
}

/**
 * Generate a new RSA key pair for agent authentication
 */
export function generateKeyPair(): KeyPair {
  const { privateKey, publicKey } = crypto.generateKeyPairSync('rsa', {
    modulusLength: 2048,
    publicKeyEncoding: {
      type: 'spki',
      format: 'pem',
    },
    privateKeyEncoding: {
      type: 'pkcs8',
      format: 'pem',
    },
  });

  return { privateKey, publicKey };
}

/**
 * Load a private key from a file path
 */
export function loadPrivateKey(keyPath: string): string {
  if (!fs.existsSync(keyPath)) {
    throw new Error(`Private key file not found: ${keyPath}`);
  }
  return fs.readFileSync(keyPath, 'utf-8');
}

/**
 * Sign a request using the agent's private key
 */
export function signRequest(
  privateKeyPem: string,
  method: string,
  path: string,
  body?: Buffer | string,
  timestamp?: string
): string {
  const ts = timestamp || Math.floor(Date.now() / 1000).toString();

  // Create signature payload
  let bodyHash = '';
  if (body) {
    const bodyBuffer = typeof body === 'string' ? Buffer.from(body) : body;
    bodyHash = crypto.createHash('sha256').update(bodyBuffer).digest('hex');
  }

  const payload = `${method}\n${path}\n${ts}\n${bodyHash}`;

  // For MVP, use HMAC with private key material
  // In production, this would use proper cryptographic signing
  const privateKey = crypto.createPrivateKey(privateKeyPem);
  const keyDer = privateKey.export({ format: 'der', type: 'pkcs8' }) as Buffer;
  const hmacKey = keyDer.slice(0, 32);

  const signature = crypto.createHmac('sha256', hmacKey).update(payload).digest();
  return signature.toString('base64');
}

