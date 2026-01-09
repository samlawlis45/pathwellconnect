'use client';

import { useState } from 'react';
import { useTrace } from '@/hooks/useTraces';
import { format, formatDistanceToNow } from 'date-fns';
import Link from 'next/link';
import { Icon } from '@iconify/react';
import { Timeline } from '@/components/timeline/Timeline';
import { DecisionTreeView } from '@/components/decision-tree/DecisionTree';

export default function TraceDetailPage({ params }: { params: { traceId: string } }) {
  const { trace, timeline, decisionTree, isLoading, error } = useTrace(params.traceId);
  const [activeTab, setActiveTab] = useState<'timeline' | 'decisions' | 'raw'>('timeline');

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-slate-400 flex items-center gap-2">
          <Icon icon="solar:refresh-outline" className="w-5 h-5 animate-spin" />
          Loading trace...
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-red-500 flex items-center gap-2">
          <Icon icon="solar:danger-triangle-outline" className="w-5 h-5" />
          Error loading trace: {error.message}
        </div>
      </div>
    );
  }

  if (!trace) {
    return (
      <div className="flex items-center justify-center min-h-[50vh]">
        <div className="text-slate-400 flex items-center gap-2">
          <Icon icon="solar:inbox-outline" className="w-5 h-5" />
          Trace not found
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <div className="flex items-center gap-3 mb-2">
            <Link href="/traces" className="text-slate-400 hover:text-pathwell-600 transition-colors">
              <Icon icon="solar:arrow-left-outline" className="w-5 h-5" />
            </Link>
            <h1 className="text-2xl font-semibold text-slate-900">Trace Detail</h1>
            <StatusBadge status={trace.status} />
          </div>
          {trace.correlation_id && (
            <p className="text-slate-500">
              Correlation ID:{' '}
              <code className="bg-pathwell-50 text-pathwell-700 px-2 py-1 rounded font-mono text-sm">
                {trace.correlation_id}
              </code>
            </p>
          )}
          <p className="text-slate-400 text-sm mt-1 font-mono">{trace.trace_id}</p>
        </div>
      </div>

      {/* Summary cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        <SummaryCard
          title="Events"
          value={trace.event_count}
          icon="solar:bolt-outline"
        />
        <SummaryCard
          title="Policy Denials"
          value={trace.policy_deny_count}
          icon="solar:shield-cross-outline"
          variant={trace.policy_deny_count > 0 ? 'danger' : 'default'}
        />
        <SummaryCard
          title="Started"
          value={format(new Date(trace.started_at), 'MMM d, HH:mm:ss')}
          icon="solar:clock-circle-outline"
          small
        />
        <SummaryCard
          title="Last Activity"
          value={formatDistanceToNow(new Date(trace.last_event_at), { addSuffix: true })}
          icon="solar:history-outline"
          small
        />
      </div>

      {/* Agent info */}
      {trace.initiating_agent_id && (
        <div className="bg-white border border-slate-200 rounded-xl p-4">
          <div className="flex items-center gap-2 text-sm text-slate-500 mb-1">
            <Icon icon="solar:user-circle-outline" className="w-4 h-4" />
            Initiating Agent
          </div>
          <div className="text-slate-900 font-mono">{trace.initiating_agent_id}</div>
        </div>
      )}

      {/* Tabs */}
      <div className="bg-white border border-slate-200 rounded-xl overflow-hidden">
        <div className="border-b border-slate-100 px-4">
          <nav className="flex gap-4">
            <TabButton
              active={activeTab === 'timeline'}
              onClick={() => setActiveTab('timeline')}
              icon="solar:timeline-down-outline"
            >
              Timeline
            </TabButton>
            <TabButton
              active={activeTab === 'decisions'}
              onClick={() => setActiveTab('decisions')}
              icon="solar:share-outline"
            >
              Decision Tree
            </TabButton>
            <TabButton
              active={activeTab === 'raw'}
              onClick={() => setActiveTab('raw')}
              icon="solar:code-square-outline"
            >
              Raw Data
            </TabButton>
          </nav>
        </div>

        <div className="p-6">
          {activeTab === 'timeline' && <Timeline events={timeline} />}
          {activeTab === 'decisions' && decisionTree && <DecisionTreeView data={decisionTree} />}
          {activeTab === 'raw' && (
            <pre className="bg-slate-50 border border-slate-200 p-4 rounded-lg overflow-auto max-h-[600px] text-sm text-slate-700 font-mono">
              {JSON.stringify({ trace, timeline, decisionTree }, null, 2)}
            </pre>
          )}
        </div>
      </div>
    </div>
  );
}

function StatusBadge({ status }: { status: string }) {
  const classes = {
    active: 'bg-emerald-50 text-emerald-700 ring-emerald-600/20',
    completed: 'bg-slate-50 text-slate-600 ring-slate-500/20',
    failed: 'bg-red-50 text-red-700 ring-red-600/20',
  }[status] || 'bg-slate-50 text-slate-600 ring-slate-500/20';

  return (
    <span className={`inline-flex items-center px-2.5 py-1 rounded-md text-xs font-medium ring-1 ring-inset ${classes}`}>
      {status}
    </span>
  );
}

function SummaryCard({
  title,
  value,
  icon,
  variant = 'default',
  small = false,
}: {
  title: string;
  value: string | number;
  icon: string;
  variant?: 'default' | 'danger';
  small?: boolean;
}) {
  const valueClasses = variant === 'danger' ? 'text-red-600' : 'text-slate-900';
  const iconClasses = variant === 'danger' ? 'text-red-500 bg-red-50' : 'text-pathwell-500 bg-pathwell-50';

  return (
    <div className="bg-white border border-slate-200 rounded-xl p-4">
      <div className="flex items-center justify-between">
        <div>
          <div className="text-sm text-slate-500">{title}</div>
          <div className={`${small ? 'text-lg' : 'text-2xl'} font-semibold mt-1 ${valueClasses}`}>
            {value}
          </div>
        </div>
        <div className={`w-10 h-10 rounded-lg ${iconClasses} flex items-center justify-center`}>
          <Icon icon={icon} className="w-5 h-5" />
        </div>
      </div>
    </div>
  );
}

function TabButton({
  active,
  onClick,
  icon,
  children,
}: {
  active: boolean;
  onClick: () => void;
  icon: string;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={`flex items-center gap-2 py-3 px-1 border-b-2 transition-colors text-sm font-medium ${
        active
          ? 'border-pathwell-600 text-pathwell-600'
          : 'border-transparent text-slate-500 hover:text-slate-700'
      }`}
    >
      <Icon icon={icon} className="w-4 h-4" />
      {children}
    </button>
  );
}
