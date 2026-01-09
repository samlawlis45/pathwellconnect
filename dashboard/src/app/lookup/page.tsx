'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { lookupByCorrelation } from '@/lib/api';
import { Icon } from '@iconify/react';

export default function LookupPage() {
  const router = useRouter();
  const [searchValue, setSearchValue] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [isSearching, setIsSearching] = useState(false);

  const handleSearch = async (e?: React.FormEvent) => {
    e?.preventDefault();
    if (!searchValue.trim()) return;

    setIsSearching(true);
    setError(null);

    try {
      const result = await lookupByCorrelation(searchValue.trim());
      if (result.trace) {
        router.push(`/traces/${result.trace.trace_id}`);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Error looking up transaction');
    } finally {
      setIsSearching(false);
    }
  };

  return (
    <div className="min-h-[70vh] flex items-center justify-center">
      <div className="w-full max-w-2xl">
        <div className="text-center mb-8">
          <div className="w-16 h-16 rounded-2xl bg-pathwell-50 flex items-center justify-center mx-auto mb-4">
            <Icon icon="solar:magnifer-zoom-in-outline" className="w-8 h-8 text-pathwell-600" />
          </div>
          <h1 className="text-3xl font-semibold text-slate-900 mb-2">
            Transaction Tracker
          </h1>
          <p className="text-slate-500 text-lg">
            Enter a reference number to track your transaction journey
          </p>
        </div>

        <form onSubmit={handleSearch} className="bg-white border border-slate-200 rounded-2xl p-8 shadow-sm">
          <div className="flex gap-3">
            <div className="flex-1 relative">
              <div className="absolute left-4 top-1/2 -translate-y-1/2 text-slate-400">
                <Icon icon="solar:magnifer-outline" className="w-5 h-5" />
              </div>
              <input
                type="text"
                value={searchValue}
                onChange={e => setSearchValue(e.target.value)}
                onKeyDown={e => e.key === 'Enter' && handleSearch()}
                placeholder="Enter correlation ID or reference number..."
                className="w-full pl-12 pr-4 py-4 bg-white border border-slate-300 rounded-xl text-slate-900 placeholder-slate-400 text-lg focus:outline-none focus:ring-2 focus:ring-pathwell-500 focus:border-transparent"
                autoFocus
              />
            </div>
            <button
              type="submit"
              disabled={isSearching || !searchValue.trim()}
              className="px-8 py-4 bg-pathwell-600 text-white rounded-xl hover:bg-pathwell-700 disabled:opacity-50 disabled:cursor-not-allowed font-semibold text-lg transition-colors"
            >
              {isSearching ? (
                <span className="flex items-center gap-2">
                  <Icon icon="solar:refresh-outline" className="w-5 h-5 animate-spin" />
                  Searching
                </span>
              ) : (
                'Track'
              )}
            </button>
          </div>

          {error && (
            <div className="mt-4 p-4 bg-red-50 border border-red-200 rounded-lg text-red-600 text-center flex items-center justify-center gap-2">
              <Icon icon="solar:danger-triangle-outline" className="w-5 h-5" />
              {error}
            </div>
          )}
        </form>

        <div className="mt-8 text-center">
          <p className="text-slate-400 text-sm mb-3">Try an example:</p>
          <div className="flex flex-wrap justify-center gap-2">
            {['PO-2024-001', 'INV-12345', 'ORDER-ABC123'].map(example => (
              <button
                key={example}
                onClick={() => setSearchValue(example)}
                className="px-4 py-2 bg-white border border-slate-200 rounded-lg text-sm text-slate-600 hover:text-pathwell-600 hover:border-pathwell-300 transition-colors font-mono"
              >
                {example}
              </button>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
