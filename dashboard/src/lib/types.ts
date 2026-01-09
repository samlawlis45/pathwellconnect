export interface TraceSummary {
  trace_id: string;
  correlation_id: string | null;
  status: 'active' | 'completed' | 'failed';
  started_at: string;
  last_event_at: string;
  event_count: number;
  policy_deny_count: number;
  initiating_agent_id: string | null;
  initiating_developer_id: string | null;
  initiating_enterprise_id: string | null;
}

export interface TraceListResponse {
  traces: TraceSummary[];
  total: number;
  limit: number;
  offset: number;
}

export interface TraceDetailResponse {
  trace: TraceSummary;
  timeline: TimelineEvent[];
  decision_tree: DecisionTree;
}

export interface TimelineEvent {
  event_id: string;
  timestamp: string;
  event_type: string;
  source_system: string;
  source_service: string;
  agent_id: string | null;
  summary: string;
  outcome: EventOutcome;
  details: Record<string, unknown>;
}

export interface EventOutcome {
  success: boolean;
  reason: string | null;
}

export interface DecisionTree {
  nodes: DecisionNode[];
  edges: DecisionEdge[];
}

export interface DecisionNode {
  id: string;
  node_type: 'identity' | 'policy' | 'action';
  label: string;
  outcome: boolean;
  timestamp: string;
  details: Record<string, unknown>;
}

export interface DecisionEdge {
  from: string;
  to: string;
  label: string | null;
}

export interface TraceQueryParams {
  correlation_id?: string;
  agent_id?: string;
  enterprise_id?: string;
  status?: string;
  from?: string;
  to?: string;
  limit?: number;
  offset?: number;
}
