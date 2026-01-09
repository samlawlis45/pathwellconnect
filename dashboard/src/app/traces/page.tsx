'use client';

import { useState } from 'react';
import { useTraces } from '@/hooks/useTraces';
import { formatDistanceToNow, format } from 'date-fns';
import Link from 'next/link';

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
          <h1 className="text-2xl font-bold text-white">Transaction Traces</h1>
          <p className="text-slate-400">Browse and filter all transaction traces</p>
        </div>
        <button
          onClick={() => refresh()}
          className="px-4 py-2 bg-pathwell-blue text-white rounded-lg hover:bg-pathwell-blue/90 transition-colors"
        >
          Refresh
        </button>
      </div>

      {/* Filters */}
      <div className="bg-slate-800 border border-slate-700 rounded-xl p-4">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div>
            <label className="block text-sm text-slate-400 mb-1">Correlation ID</label>
            <input
              type="text"
              value={filters.correlation_id}
              onChange={e => handleFilterChange('correlation_id', e.target.value)}
              placeholder="e.g., PO-2024-001"
              className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-pathwell-blue"
            />
          </div>
          <div>
            <label className="block text-sm text-slate-400 mb-1">Agent ID</label>
            <input
              type="text"
              value={filters.agent_id}
              onChange={e => handleFilterChange('agent_id', e.target.value)}
              placeholder="e.g., sales-agent"
              className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-white placeholder-slate-500 focus:outline-none focus:border-pathwell-blue"
            />
          </div>
          <div>
            <label className="block text-sm text-slate-400 mb-1">Status</label>
            <select
              value={filters.status}
              onChange={e => handleFilterChange('status', e.target.value)}
              className="w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-white focus:outline-none focus:border-pathwell-blue"
            >
              <option value="">All</option>
              <option value="active">Active</option>
              <option value="completed">Completed</option>
              <option value="failed">Failed</option>
            </select>
          </div>
          <div className="flex items-end">
            <button
              onClick={() => setFilters({ correlation_id: '', agent_id: '', status: '', limit: 20, offset: 0 })}
              className="w-full px-4 py-2 border border-slate-600 text-slate-300 rounded-lg hover:bg-slate-700 transition-colors"
            >
              Clear Filters
            </button>
          </div>
        </div>
      </div>

      {/* Results */}
      <div className="bg-slate-800 border border-slate-700 rounded-xl overflow-hidden">
        <div className="px-6 py-4 border-b border-slate-700">
          <span className="text-slate-400">{total} traces found</span>
        </div>

        {isLoading ? (
          <div className="p-8 text-center text-slate-400">Loading...</div>
        ) : error ? (
          <div className="p-8 text-center text-red-400">Failed to load traces: {error.message}</div>
        ) : traces.length === 0 ? (
          <div className="p-8 text-center text-slate-400">No traces match your filters</div>
        ) : (
          <>
            <table className="w-full">
              <thead className="bg-slate-900/50">
                <tr>
                  <th className="text-left px-6 py-3 text-sm text-slate-400 font-medium">Reference</th>
                  <th className="text-left px-6 py-3 text-sm text-slate-400 font-medium">Agent</th>
                  <th className="text-left px-6 py-3 text-sm text-slate-400 font-medium">Status</th>
                  <th className="text-left px-6 py-3 text-sm text-slate-400 font-medium">Events</th>
                  <th className="text-left px-6 py-3 text-sm text-slate-400 font-medium">Started</th>
                  <th className="text-left px-6 py-3 text-sm text-slate-400 font-medium">Last Activity</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-700">
                {traces.map(trace => (
                  <tr key={trace.trace_id} className="hover:bg-slate-700/30 transition-colors">
                    <td className="px-6 py-4">
                      <Link
                        href={`/traces/${trace.trace_id}`}
                        className="text-pathwell-blue hover:underline font-mono"
                      >
                        {trace.correlation_id || trace.trace_id.slice(0, 12)}
                      </Link>
                    </td>
                    <td className="px-6 py-4 text-slate-300">
                      {trace.initiating_agent_id || '-'}
                    </td>
                    <td className="px-6 py-4">
                      <StatusBadge status={trace.status} />
                    </td>
                    <td className="px-6 py-4">
                      <span className="text-white">{trace.event_count}</span>
                      {trace.policy_deny_count > 0 && (
                        <span className="text-red-400 ml-2">({trace.policy_deny_count} denied)</span>
                      )}
                    </td>
                    <td className="px-6 py-4 text-slate-400 text-sm">
                      {format(new Date(trace.started_at), 'MMM d, HH:mm')}
                    </td>
                    <td className="px-6 py-4 text-slate-400 text-sm">
                      {formatDistanceToNow(new Date(trace.last_event_at), { addSuffix: true })}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="px-6 py-4 border-t border-slate-700 flex items-center justify-between">
                <span className="text-sm text-slate-400">
                  Page {currentPage} of {totalPages}
                </span>
                <div className="flex gap-2">
                  <button
                    onClick={() => handlePageChange(filters.offset - filters.limit)}
                    disabled={filters.offset === 0}
                    className="px-3 py-1 border border-slate-600 rounded text-slate-300 disabled:opacity-50 disabled:cursor-not-allowed hover:bg-slate-700"
                  >
                    Previous
                  </button>
                  <button
                    onClick={() => handlePageChange(filters.offset + filters.limit)}
                    disabled={filters.offset + filters.limit >= total}
                    className="px-3 py-1 border border-slate-600 rounded text-slate-300 disabled:opacity-50 disabled:cursor-not-allowed hover:bg-slate-700"
                  >
                    Next
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
