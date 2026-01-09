'use client';

import { useTraces } from '@/hooks/useTraces';
import { formatDistanceToNow } from 'date-fns';
import Link from 'next/link';

export default function DashboardPage() {
  const { traces, total, isLoading, error } = useTraces({ limit: 5 });

  const stats = {
    totalTraces: total,
    activeTraces: traces.filter(t => t.status === 'active').length,
    deniedRequests: traces.reduce((sum, t) => sum + t.policy_deny_count, 0),
    recentEvents: traces.reduce((sum, t) => sum + t.event_count, 0),
  };

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold text-white mb-2">Intelligent Ledger</h1>
        <p className="text-slate-400">Transaction lineage explorer for AI agent governance</p>
      </div>

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <StatCard title="Total Traces" value={stats.totalTraces} />
        <StatCard title="Active" value={stats.activeTraces} color="green" />
        <StatCard title="Policy Denials" value={stats.deniedRequests} color="red" />
        <StatCard title="Recent Events" value={stats.recentEvents} color="blue" />
      </div>

      {/* Quick Actions */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Link
          href="/lookup"
          className="bg-slate-800 border border-slate-700 rounded-xl p-6 hover:border-pathwell-blue transition-colors group"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-lg bg-pathwell-blue/20 flex items-center justify-center group-hover:bg-pathwell-blue/30 transition-colors">
              <svg className="w-6 h-6 text-pathwell-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-white">Track Transaction</h3>
              <p className="text-slate-400">Enter a reference number to track its journey</p>
            </div>
          </div>
        </Link>

        <Link
          href="/traces"
          className="bg-slate-800 border border-slate-700 rounded-xl p-6 hover:border-pathwell-blue transition-colors group"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-lg bg-pathwell-blue/20 flex items-center justify-center group-hover:bg-pathwell-blue/30 transition-colors">
              <svg className="w-6 h-6 text-pathwell-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 10h16M4 14h16M4 18h16" />
              </svg>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-white">Browse All Traces</h3>
              <p className="text-slate-400">View and filter all transaction traces</p>
            </div>
          </div>
        </Link>
      </div>

      {/* Recent Traces */}
      <div className="bg-slate-800 border border-slate-700 rounded-xl overflow-hidden">
        <div className="px-6 py-4 border-b border-slate-700 flex items-center justify-between">
          <h2 className="text-lg font-semibold text-white">Recent Traces</h2>
          <Link href="/traces" className="text-pathwell-blue hover:underline text-sm">
            View all
          </Link>
        </div>
        {isLoading ? (
          <div className="p-6 text-center text-slate-400">Loading...</div>
        ) : error ? (
          <div className="p-6 text-center text-red-400">Failed to load traces</div>
        ) : traces.length === 0 ? (
          <div className="p-6 text-center text-slate-400">No traces yet</div>
        ) : (
          <div className="divide-y divide-slate-700">
            {traces.map(trace => (
              <Link
                key={trace.trace_id}
                href={`/traces/${trace.trace_id}`}
                className="block px-6 py-4 hover:bg-slate-700/50 transition-colors"
              >
                <div className="flex items-center justify-between">
                  <div>
                    <div className="flex items-center gap-2">
                      <span className="font-mono text-sm text-slate-300">
                        {trace.correlation_id || trace.trace_id.slice(0, 8)}
                      </span>
                      <StatusBadge status={trace.status} />
                    </div>
                    <div className="text-sm text-slate-400 mt-1">
                      {trace.event_count} events
                      {trace.policy_deny_count > 0 && (
                        <span className="text-red-400 ml-2">
                          {trace.policy_deny_count} denied
                        </span>
                      )}
                    </div>
                  </div>
                  <div className="text-sm text-slate-500">
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

function StatCard({ title, value, color = 'default' }: { title: string; value: number; color?: string }) {
  const colorClasses = {
    default: 'text-white',
    green: 'text-green-400',
    red: 'text-red-400',
    blue: 'text-pathwell-blue',
  }[color] || 'text-white';

  return (
    <div className="bg-slate-800 border border-slate-700 rounded-xl p-6">
      <div className="text-sm text-slate-400">{title}</div>
      <div className={`text-3xl font-bold mt-1 ${colorClasses}`}>{value}</div>
    </div>
  );
}

function StatusBadge({ status }: { status: string }) {
  const colorClasses = {
    active: 'bg-green-400/20 text-green-400',
    completed: 'bg-slate-400/20 text-slate-400',
    failed: 'bg-red-400/20 text-red-400',
  }[status] || 'bg-slate-400/20 text-slate-400';

  return (
    <span className={`px-2 py-0.5 rounded-full text-xs font-medium ${colorClasses}`}>
      {status}
    </span>
  );
}
