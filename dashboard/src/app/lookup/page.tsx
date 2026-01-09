'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { lookupByCorrelation } from '@/lib/api';

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
          <h1 className="text-4xl font-bold text-white mb-3">
            Transaction Tracker
          </h1>
          <p className="text-slate-400 text-lg">
            Enter a reference number to track your transaction journey
          </p>
        </div>

        <form onSubmit={handleSearch} className="bg-slate-800 border border-slate-700 rounded-2xl p-8 shadow-2xl">
          <div className="flex gap-4">
            <div className="flex-1 relative">
              <div className="absolute left-4 top-1/2 -translate-y-1/2 text-slate-400">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
              </div>
              <input
                type="text"
                value={searchValue}
                onChange={e => setSearchValue(e.target.value)}
                onKeyDown={e => e.key === 'Enter' && handleSearch()}
                placeholder="Enter correlation ID or reference number..."
                className="w-full pl-12 pr-4 py-4 bg-slate-900 border border-slate-600 rounded-xl text-white placeholder-slate-500 text-lg focus:outline-none focus:border-pathwell-blue focus:ring-1 focus:ring-pathwell-blue"
                autoFocus
              />
            </div>
            <button
              type="submit"
              disabled={isSearching || !searchValue.trim()}
              className="px-8 py-4 bg-pathwell-blue text-white rounded-xl hover:bg-pathwell-blue/90 disabled:opacity-50 disabled:cursor-not-allowed font-semibold text-lg transition-colors"
            >
              {isSearching ? (
                <span className="flex items-center gap-2">
                  <svg className="animate-spin h-5 w-5" viewBox="0 0 24 24">
                    <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                    <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
                  </svg>
                  Searching
                </span>
              ) : (
                'Track'
              )}
            </button>
          </div>

          {error && (
            <div className="mt-4 p-4 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-center">
              {error}
            </div>
          )}
        </form>

        <div className="mt-8 text-center">
          <p className="text-slate-500 text-sm mb-2">Examples:</p>
          <div className="flex flex-wrap justify-center gap-2">
            {['PO-2024-001', 'INV-12345', 'ORDER-ABC123'].map(example => (
              <button
                key={example}
                onClick={() => setSearchValue(example)}
                className="px-3 py-1 bg-slate-800 border border-slate-700 rounded-full text-sm text-slate-400 hover:text-white hover:border-slate-600 transition-colors"
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
