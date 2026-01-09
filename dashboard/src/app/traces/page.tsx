'use client';

import { useState } from 'react';
import { useTraces } from '@/hooks/useTraces';
import { formatDistanceToNow, format } from 'date-fns';
import Link from 'next/link';
import { Icon } from '@iconify/react';

export default function TracesPage() {
  const [filters, setFilters] = useState({
    correlation_id: '',
    agent_id: '',
    status: '',
    limit: 20,
    offset: 0,
  });

  const { traces, total, isLoading, error, refresh } = useTraces(filters);

  const handleFilterChange = (key: string, value: string) => {
    setFilters(prev => ({ ...prev, [key]: value, offset: 0 }));
  };

  const handlePageChange = (newOffset: number) => {
    setFilters(prev => ({ ...prev, offset: newOffset }));
  };

  const totalPages = Math.ceil(total / filters.limit);
  const currentPage = Math.floor(filters.offset / filters.limit) + 1;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold text-slate-900">Transaction Traces</h1>
          <p className="text-slate-500 mt-1">Browse and filter all transaction traces</p>
        </div>
        <button
          onClick={() => refresh()}
          className="inline-flex items-center gap-2 px-4 py-2 bg-pathwell-600 text-white rounded-lg hover:bg-pathwell-700 transition-colors font-medium text-sm"
        >
          <Icon icon="solar:refresh-outline" className="w-4 h-4" />
          Refresh
        </button>
      </div>

      {/* Filters */}
      <div className="bg-white border border-slate-200 rounded-xl p-5">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div>
            <label className="block text-sm font-medium text-slate-700 mb-1.5">Correlation ID</label>
            <input
              type="text"
              value={filters.correlation_id}
              onChange={e => handleFilterChange('correlation_id', e.target.value)}
              placeholder="e.g., PO-2024-001"
              className="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-pathwell-500 focus:border-transparent"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-slate-700 mb-1.5">Agent ID</label>
            <input
              type="text"
              value={filters.agent_id}
              onChange={e => handleFilterChange('agent_id', e.target.value)}
              placeholder="e.g., sales-agent"
              className="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-pathwell-500 focus:border-transparent"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-slate-700 mb-1.5">Status</label>
            <select
              value={filters.status}
              onChange={e => handleFilterChange('status', e.target.value)}
              className="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-slate-900 focus:outline-none focus:ring-2 focus:ring-pathwell-500 focus:border-transparent"
            >
              <option value="">All statuses</option>
              <option value="active">Active</option>
              <option value="completed">Completed</option>
              <option value="failed">Failed</option>
            </select>
          </div>
          <div className="flex items-end">
            <button
              onClick={() => setFilters({ correlation_id: '', agent_id: '', status: '', limit: 20, offset: 0 })}
              className="w-full px-4 py-2 border border-slate-300 text-slate-700 rounded-lg hover:bg-slate-50 transition-colors font-medium text-sm"
            >
              Clear Filters
            </button>
          </div>
        </div>
      </div>

      {/* Results */}
      <div className="bg-white border border-slate-200 rounded-xl overflow-hidden">
        <div className="px-6 py-4 border-b border-slate-100 flex items-center justify-between">
          <span className="text-sm text-slate-500">{total} traces found</span>
        </div>

        {isLoading ? (
          <div className="p-12 text-center text-slate-400">
            <Icon icon="solar:refresh-outline" className="w-6 h-6 mx-auto mb-2 animate-spin" />
            Loading traces...
          </div>
        ) : error ? (
          <div className="p-12 text-center text-red-500">
            <Icon icon="solar:danger-triangle-outline" className="w-6 h-6 mx-auto mb-2" />
            Failed to load traces
          </div>
        ) : traces.length === 0 ? (
          <div className="p-12 text-center text-slate-400">
            <Icon icon="solar:inbox-outline" className="w-8 h-8 mx-auto mb-2" />
            No traces match your filters
          </div>
        ) : (
          <>
            <table className="w-full">
              <thead className="bg-slate-50">
                <tr>
                  <th className="text-left px-6 py-3 text-xs font-medium text-slate-500 uppercase tracking-wider">Reference</th>
                  <th className="text-left px-6 py-3 text-xs font-medium text-slate-500 uppercase tracking-wider">Agent</th>
                  <th className="text-left px-6 py-3 text-xs font-medium text-slate-500 uppercase tracking-wider">Status</th>
                  <th className="text-left px-6 py-3 text-xs font-medium text-slate-500 uppercase tracking-wider">Events</th>
                  <th className="text-left px-6 py-3 text-xs font-medium text-slate-500 uppercase tracking-wider">Started</th>
                  <th className="text-left px-6 py-3 text-xs font-medium text-slate-500 uppercase tracking-wider">Last Activity</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100">
                {traces.map(trace => (
                  <tr key={trace.trace_id} className="hover:bg-slate-50 transition-colors">
                    <td className="px-6 py-4">
                      <Link
                        href={`/traces/${trace.trace_id}`}
                        className="text-pathwell-600 hover:text-pathwell-700 font-mono text-sm font-medium"
                      >
                        {trace.correlation_id || trace.trace_id.slice(0, 12)}
                      </Link>
                    </td>
                    <td className="px-6 py-4 text-slate-700 text-sm">
                      {trace.initiating_agent_id || '-'}
                    </td>
                    <td className="px-6 py-4">
                      <StatusBadge status={trace.status} />
                    </td>
                    <td className="px-6 py-4">
                      <span className="text-slate-900 text-sm">{trace.event_count}</span>
                      {trace.policy_deny_count > 0 && (
                        <span className="text-red-500 text-sm ml-2">({trace.policy_deny_count} denied)</span>
                      )}
                    </td>
                    <td className="px-6 py-4 text-slate-500 text-sm">
                      {format(new Date(trace.started_at), 'MMM d, HH:mm')}
                    </td>
                    <td className="px-6 py-4 text-slate-500 text-sm">
                      {formatDistanceToNow(new Date(trace.last_event_at), { addSuffix: true })}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="px-6 py-4 border-t border-slate-100 flex items-center justify-between">
                <span className="text-sm text-slate-500">
                  Page {currentPage} of {totalPages}
                </span>
                <div className="flex gap-2">
                  <button
                    onClick={() => handlePageChange(filters.offset - filters.limit)}
                    disabled={filters.offset === 0}
                    className="inline-flex items-center gap-1 px-3 py-1.5 border border-slate-300 rounded-lg text-sm text-slate-700 disabled:opacity-50 disabled:cursor-not-allowed hover:bg-slate-50"
                  >
                    <Icon icon="solar:arrow-left-outline" className="w-4 h-4" />
                    Previous
                  </button>
                  <button
                    onClick={() => handlePageChange(filters.offset + filters.limit)}
                    disabled={filters.offset + filters.limit >= total}
                    className="inline-flex items-center gap-1 px-3 py-1.5 border border-slate-300 rounded-lg text-sm text-slate-700 disabled:opacity-50 disabled:cursor-not-allowed hover:bg-slate-50"
                  >
                    Next
                    <Icon icon="solar:arrow-right-outline" className="w-4 h-4" />
                  </button>
                </div>
              </div>
            )}
          </>
        )}
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
