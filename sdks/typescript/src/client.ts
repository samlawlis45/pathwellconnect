import fetch, { RequestInit, Response } from 'node-fetch';
import { URL } from 'url';
import { loadPrivateKey, signRequest } from './auth';

export interface PathwellClientOptions {
  agentId: string;
  privateKeyPath: string;
  proxyUrl?: string;
  targetUrl?: string;
}

export class PathwellClient {
  private agentId: string;
  private privateKey: string;
  private proxyUrl: string;
  private targetUrl: string;

  constructor(options: PathwellClientOptions) {
    this.agentId = options.agentId;
    this.privateKey = loadPrivateKey(options.privateKeyPath);
    this.proxyUrl = (options.proxyUrl || 'http://localhost:8080').replace(/\/$/, '');
    this.targetUrl = options.targetUrl || this.proxyUrl;
  }

  /**
   * Make an authenticated request through Pathwell proxy
   */
  async call(
    method: string,
    url: string,
    options: {
      headers?: Record<string, string>;
      body?: any;
    } & RequestInit = {}
  ): Promise<Response> {
    const { headers = {}, body, ...fetchOptions } = options;

    // Prepare headers
    const reqHeaders: Record<string, string> = { ...headers };
    reqHeaders['X-Pathwell-Agent-ID'] = this.agentId;

    // Prepare body
    let bodyBytes: Buffer | undefined;
    if (body !== undefined) {
      if (typeof body === 'object') {
        bodyBytes = Buffer.from(JSON.stringify(body));
        reqHeaders['Content-Type'] = reqHeaders['Content-Type'] || 'application/json';
      } else if (typeof body === 'string') {
        bodyBytes = Buffer.from(body);
      } else if (Buffer.isBuffer(body)) {
        bodyBytes = body;
      }
    }

    // Extract path from URL for signing
    const parsedUrl = new URL(url);
    let path = parsedUrl.pathname;
    if (parsedUrl.search) {
      path += parsedUrl.search;
    }

    // Sign request
    const timestamp = Math.floor(Date.now() / 1000).toString();
    const signature = signRequest(
      this.privateKey,
      method,
      path,
      bodyBytes,
      timestamp
    );
    reqHeaders['X-Pathwell-Signature'] = signature;
    reqHeaders['X-Pathwell-Timestamp'] = timestamp;

    // Make request through proxy
    const proxyUrl = `${this.proxyUrl}${path}`;

    return fetch(proxyUrl, {
      method,
      headers: reqHeaders,
      body: bodyBytes,
      ...fetchOptions,
    });
  }

  async get(url: string, options?: RequestInit): Promise<Response> {
    return this.call('GET', url, options);
  }

  async post(url: string, body?: any, options?: RequestInit): Promise<Response> {
    return this.call('POST', url, { ...options, body });
  }

  async put(url: string, body?: any, options?: RequestInit): Promise<Response> {
    return this.call('PUT', url, { ...options, body });
  }

  async patch(url: string, body?: any, options?: RequestInit): Promise<Response> {
    return this.call('PATCH', url, { ...options, body });
  }

  async delete(url: string, options?: RequestInit): Promise<Response> {
    return this.call('DELETE', url, options);
  }
}

