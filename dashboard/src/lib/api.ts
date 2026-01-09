import type {
  TraceListResponse,
  TraceDetailResponse,
  TimelineEvent,
  DecisionTree,
  TraceQueryParams,
} from './types';

const API_BASE = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3003';

export async function fetchTraces(params: TraceQueryParams = {}): Promise<TraceListResponse> {
  const searchParams = new URLSearchParams();

  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== null && value !== '') {
      searchParams.set(key, String(value));
    }
  });

  const queryString = searchParams.toString();
  const url = `${API_BASE}/v1/traces${queryString ? `?${queryString}` : ''}`;

  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to fetch traces: ${response.statusText}`);
  }
  return response.json();
}

export async function fetchTrace(traceId: string): Promise<TraceDetailResponse> {
  const response = await fetch(`${API_BASE}/v1/traces/${traceId}`);
  if (!response.ok) {
    throw new Error(`Failed to fetch trace: ${response.statusText}`);
  }
  return response.json();
}

export async function fetchTimeline(traceId: string): Promise<TimelineEvent[]> {
  const response = await fetch(`${API_BASE}/v1/traces/${traceId}/timeline`);
  if (!response.ok) {
    throw new Error(`Failed to fetch timeline: ${response.statusText}`);
  }
  return response.json();
}

export async function fetchDecisionTree(traceId: string): Promise<DecisionTree> {
  const response = await fetch(`${API_BASE}/v1/traces/${traceId}/decisions`);
  if (!response.ok) {
    throw new Error(`Failed to fetch decision tree: ${response.statusText}`);
  }
  return response.json();
}

export async function lookupByCorrelation(correlationId: string): Promise<TraceDetailResponse> {
  const response = await fetch(`${API_BASE}/v1/lookup/${encodeURIComponent(correlationId)}`);
  if (!response.ok) {
    if (response.status === 404) {
      throw new Error('No transaction found with this reference number');
    }
    throw new Error(`Failed to lookup correlation: ${response.statusText}`);
  }
  return response.json();
}
