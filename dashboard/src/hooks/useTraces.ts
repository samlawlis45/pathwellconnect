'use client';

import useSWR from 'swr';
import { fetchTraces, fetchTrace } from '@/lib/api';
import { useTenant } from '@/contexts/TenantContext';
import type { TraceQueryParams, TraceListResponse, TraceDetailResponse } from '@/lib/types';

export function useTraces(params: TraceQueryParams = {}) {
  const { currentTenant } = useTenant();

  const mergedParams = {
    ...params,
    enterprise_id: params.enterprise_id || currentTenant.id,
  };

  const key = ['traces', JSON.stringify(mergedParams), currentTenant.id];

  const { data, error, isLoading, mutate } = useSWR<TraceListResponse>(
    key,
    () => fetchTraces(mergedParams),
    {
      refreshInterval: 5000, // Auto-refresh every 5 seconds
    }
  );

  return {
    traces: data?.traces || [],
    total: data?.total || 0,
    limit: data?.limit || 50,
    offset: data?.offset || 0,
    isLoading,
    error,
    refresh: mutate,
  };
}

export function useTrace(traceId: string) {
  const { data, error, isLoading } = useSWR<TraceDetailResponse>(
    traceId ? ['trace', traceId] : null,
    () => fetchTrace(traceId),
    {
      refreshInterval: 5000,
    }
  );

  return {
    trace: data?.trace,
    timeline: data?.timeline || [],
    decisionTree: data?.decision_tree,
    isLoading,
    error,
  };
}
