'use client';

import { useState } from 'react';
import { useTrace } from '@/hooks/useTraces';
import { format, formatDistanceToNow } from 'date-fns';
import Link from 'next/link';
import { Timeline } from '@/components/timeline/Timeline';
import { DecisionTreeView } from '@/components/decision-tree/DecisionTree';

export default function TraceDetailPage({ params }: { params: { traceId: string } }) {
  const { trace, timeline, decisionTree, isLoading, error } = useTrace(params.traceId);
  const [activeTab, setActiveTab] = useState<'timeline' | 'decisions' | 'raw'>('timeline');

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-slate-400">Loading trace...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-red-400">Error loading trace: {error.message}</div>
      </div>
    );
  }

  if (!trace) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-slate-400">Trace not found</div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <div className="flex items-center gap-3 mb-2">
            <Link href="/traces" className="text-slate-400 hover:text-white">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
            </Link>
            <h1 className="text-2xl font-bold text-white">Trace Detail</h1>
            <StatusBadge status={trace.status} />
          </div>
          {trace.correlation_id && (
            <p className="text-slate-400">
              Correlation ID:{' '}
              <code className="bg-slate-800 px-2 py-1 rounded text-pathwell-blue">
                {trace.correlation_id}
              </code>
            </p>
          )}
          <p className="text-slate-500 text-sm mt-1 font-mono">{trace.trace_id}</p>
        </div>
      </div>

      {/* Summary cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <SummaryCard title="Events" value={trace.event_count} />
        <SummaryCard
          title="Policy Denials"
          value={trace.policy_deny_count}
          variant={trace.policy_deny_count > 0 ? 'danger' : 'default'}
        />
        <SummaryCard
          title="Started"
          value={format(new Date(trace.started_at), 'MMM d, HH:mm:ss')}
          small
        />
        <SummaryCard
          title="Last Activity"
          value={formatDistanceToNow(new Date(trace.last_event_at), { addSuffix: true })}
          small
        />
      </div>

      {/* Agent info */}
      {trace.initiating_agent_id && (
        <div className="bg-slate-800 border border-slate-700 rounded-xl p-4">
          <div className="text-sm text-slate-400 mb-1">Initiating Agent</div>
          <div className="text-white font-mono">{trace.initiating_agent_id}</div>
        </div>
      )}

      {/* Tabs */}
      <div className="bg-slate-800 border border-slate-700 rounded-xl overflow-hidden">
        <div className="border-b border-slate-700 px-4">
          <nav className="flex gap-4">
            <TabButton active={activeTab === 'timeline'} onClick={() => setActiveTab('timeline')}>
              Timeline
            </TabButton>
            <TabButton active={activeTab === 'decisions'} onClick={() => setActiveTab('decisions')}>
              Decision Tree
            </TabButton>
            <TabButton active={activeTab === 'raw'} onClick={() => setActiveTab('raw')}>
              Raw Data
            </TabButton>
          </nav>
        </div>

        <div className="p-6">
          {activeTab === 'timeline' && <Timeline events={timeline} />}
          {activeTab === 'decisions' && decisionTree && <DecisionTreeView data={decisionTree} />}
          {activeTab === 'raw' && (
            <pre className="bg-slate-900 p-4 rounded-lg overflow-auto max-h-[600px] text-sm text-slate-300">
              {JSON.stringify({ trace, timeline, decisionTree }, null, 2)}
            </pre>
          )}
        </div>
      </div>
    </div>
  );
}

function StatusBadge({ status }: { status: string }) {
  const colorClasses = {
    active: 'bg-green-400/20 text-green-400 border-green-400/30',
    completed: 'bg-slate-400/20 text-slate-400 border-slate-400/30',
    failed: 'bg-red-400/20 text-red-400 border-red-400/30',
  }[status] || 'bg-slate-400/20 text-slate-400 border-slate-400/30';

  return (
    <span className={`px-3 py-1 rounded-full text-sm font-medium border ${colorClasses}`}>
      {status}
    </span>
  );
}

function SummaryCard({
  title,
  value,
  variant = 'default',
  small = false,
}: {
  title: string;
  value: string | number;
  variant?: 'default' | 'danger';
  small?: boolean;
}) {
  const valueClasses = variant === 'danger' ? 'text-red-400' : 'text-white';

  return (
    <div className="bg-slate-800 border border-slate-700 rounded-xl p-4">
      <div className="text-sm text-slate-400">{title}</div>
      <div className={`${small ? 'text-lg' : 'text-2xl'} font-bold mt-1 ${valueClasses}`}>
        {value}
      </div>
    </div>
  );
}

function TabButton({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={`py-3 px-1 border-b-2 transition-colors ${
        active
          ? 'border-pathwell-blue text-white'
          : 'border-transparent text-slate-400 hover:text-slate-200'
      }`}
    >
      {children}
    </button>
  );
}
