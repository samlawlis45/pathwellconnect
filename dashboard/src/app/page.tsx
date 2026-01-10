'use client';

import { useTraces } from '@/hooks/useTraces';
import { useTenant } from '@/contexts/TenantContext';
import { formatDistanceToNow } from 'date-fns';
import Link from 'next/link';
import { Icon } from '@iconify/react';

export default function DashboardPage() {
  const { currentTenant } = useTenant();
  const { traces, total, isLoading, error } = useTraces({ limit: 5 });

  const stats = {
    totalTraces: total,
    activeTraces: traces.filter(t => t.status === 'active').length,
    deniedRequests: traces.reduce((sum, t) => sum + t.policy_deny_count, 0),
    recentEvents: traces.reduce((sum, t) => sum + t.event_count, 0),
  };

  return (
    <div className="space-y-8">
      {/* Organization Header */}
      <div className="flex items-start justify-between">
        <div>
          <div className="flex items-center gap-3 mb-1">
            <h1 className="text-2xl font-semibold text-slate-900">Intelligent Ledger</h1>
            {currentTenant && (
              <span className="px-2 py-1 bg-pathwell-50 text-pathwell-700 text-xs font-medium rounded-md">
                {currentTenant.name}
              </span>
            )}
          </div>
          <p className="text-slate-500">
            Transaction lineage explorer for AI agent governance
          </p>
        </div>
      </div>

      {/* Systems Overview */}
      {currentTenant?.systems && currentTenant.systems.length > 0 && (
        <div className="bg-gradient-to-r from-slate-50 to-white border border-slate-200 rounded-xl p-5">
          <div className="flex items-center gap-2 text-sm text-slate-500 mb-3">
            <Icon icon="solar:server-path-outline" className="w-4 h-4" />
            Connected Systems
          </div>
          <div className="flex flex-wrap gap-2">
            {currentTenant.systems.map((system) => (
              <span
                key={system}
                className="px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-sm text-slate-700 font-medium"
              >
                {system}
              </span>
            ))}
          </div>
        </div>
      )}

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <StatCard
          title="Total Traces"
          value={stats.totalTraces}
          icon="solar:chart-square-outline"
        />
        <StatCard
          title="Active"
          value={stats.activeTraces}
          icon="solar:play-circle-outline"
          accent="emerald"
        />
        <StatCard
          title="Policy Denials"
          value={stats.deniedRequests}
          icon="solar:shield-cross-outline"
          accent="red"
        />
        <StatCard
          title="Recent Events"
          value={stats.recentEvents}
          icon="solar:bolt-outline"
          accent="blue"
        />
      </div>

      {/* Quick Actions */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Link
          href="/lookup"
          className="bg-white border border-slate-200 rounded-xl p-6 hover:border-pathwell-400 hover:shadow-sm transition-all group"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-xl bg-pathwell-50 flex items-center justify-center group-hover:bg-pathwell-100 transition-colors">
              <Icon icon="solar:magnifer-outline" className="w-6 h-6 text-pathwell-600" />
            </div>
            <div>
              <h3 className="font-semibold text-slate-900">Track Transaction</h3>
              <p className="text-slate-500 text-sm mt-0.5">Enter a PO, SO, or invoice number to track its journey</p>
            </div>
          </div>
        </Link>

        <Link
          href="/traces"
          className="bg-white border border-slate-200 rounded-xl p-6 hover:border-pathwell-400 hover:shadow-sm transition-all group"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-xl bg-pathwell-50 flex items-center justify-center group-hover:bg-pathwell-100 transition-colors">
              <Icon icon="solar:list-outline" className="w-6 h-6 text-pathwell-600" />
            </div>
            <div>
              <h3 className="font-semibold text-slate-900">Browse All Traces</h3>
              <p className="text-slate-500 text-sm mt-0.5">View and filter all transaction traces</p>
            </div>
          </div>
        </Link>
      </div>

      {/* Recent Traces */}
      <div className="bg-white border border-slate-200 rounded-xl overflow-hidden">
        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
          <h2 className="font-semibold text-slate-900">Recent Traces</h2>
          <Link href="/traces" className="text-pathwell-600 hover:text-pathwell-700 text-sm font-medium">
            View all
          </Link>
        </div>
        {isLoading ? (
          <div className="p-8 text-center text-slate-400">
            <Icon icon="solar:refresh-outline" className="w-6 h-6 mx-auto mb-2 animate-spin" />
            Loading...
          </div>
        ) : error ? (
          <div className="p-8 text-center text-red-500">
            <Icon icon="solar:danger-triangle-outline" className="w-6 h-6 mx-auto mb-2" />
            Failed to load traces
          </div>
        ) : traces.length === 0 ? (
          <div className="p-8 text-center text-slate-400">
            <Icon icon="solar:inbox-outline" className="w-8 h-8 mx-auto mb-2" />
            No traces yet
          </div>
        ) : (
          <div className="divide-y divide-slate-100">
            {traces.map(trace => (
              <Link
                key={trace.trace_id}
                href={`/traces/${trace.trace_id}`}
                className="block px-6 py-4 hover:bg-slate-50 transition-colors"
              >
                <div className="flex items-center justify-between">
                  <div>
                    <div className="flex items-center gap-2">
                      <span className="font-mono text-sm font-medium text-slate-900">
                        {trace.correlation_id || trace.trace_id.slice(0, 8)}
                      </span>
                      <StatusBadge status={trace.status} />
                      {trace.enterprise_id && (
                        <span className="px-2 py-0.5 bg-slate-100 text-slate-600 text-xs rounded">
                          {trace.enterprise_id}
                        </span>
                      )}
                    </div>
                    <div className="text-sm text-slate-500 mt-1">
                      {trace.event_count} events
                      {trace.policy_deny_count > 0 && (
                        <span className="text-red-500 ml-2">
                          · {trace.policy_deny_count} denied
                        </span>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2 text-sm text-slate-400">
                    <Icon icon="solar:clock-circle-outline" className="w-4 h-4" />
                    {formatDistanceToNow(new Date(trace.last_event_at), { addSuffix: true })}
                  </div>
                </div>
              </Link>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function StatCard({
  title,
  value,
  icon,
  accent = 'slate'
}: {
  title: string;
  value: number;
  icon: string;
  accent?: 'slate' | 'emerald' | 'red' | 'blue';
}) {
  const accentClasses = {
    slate: 'text-slate-900',
    emerald: 'text-emerald-600',
    red: 'text-red-600',
    blue: 'text-pathwell-600',
  }[accent];

  const iconBgClasses = {
    slate: 'bg-slate-100',
    emerald: 'bg-emerald-50',
    red: 'bg-red-50',
    blue: 'bg-pathwell-50',
  }[accent];

  const iconColorClasses = {
    slate: 'text-slate-500',
    emerald: 'text-emerald-500',
    red: 'text-red-500',
    blue: 'text-pathwell-500',
  }[accent];

  return (
    <div className="bg-white border border-slate-200 rounded-xl p-5">
      <div className="flex items-center justify-between">
        <div>
          <div className="text-sm text-slate-500">{title}</div>
          <div className={`text-2xl font-semibold mt-1 ${accentClasses}`}>{value}</div>
        </div>
        <div className={`w-10 h-10 rounded-lg ${iconBgClasses} flex items-center justify-center`}>
          <Icon icon={icon} className={`w-5 h-5 ${iconColorClasses}`} />
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
    <span className={`inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium ring-1 ring-inset ${classes}`}>
      {status}
    </span>
  );
}
