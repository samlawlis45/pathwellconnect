'use client';

import { useState } from 'react';
import { format } from 'date-fns';
import type { TimelineEvent } from '@/lib/types';

interface TimelineProps {
  events: TimelineEvent[];
}

export function Timeline({ events }: TimelineProps) {
  const [expandedEvent, setExpandedEvent] = useState<string | null>(null);

  if (events.length === 0) {
    return (
      <div className="text-center text-slate-400 py-8">
        No events recorded yet
      </div>
    );
  }

  return (
    <div className="relative">
      {/* Vertical line */}
      <div className="absolute left-[23px] top-4 bottom-4 w-0.5 bg-slate-700" />

      <div className="space-y-4">
        {events.map((event, index) => (
          <div key={event.event_id} className="relative flex items-start gap-4">
            {/* Timeline node */}
            <div
              className={`relative z-10 w-12 h-12 rounded-full flex items-center justify-center shrink-0 ${
                event.outcome.success
                  ? 'bg-green-500/20 border-2 border-green-500'
                  : 'bg-red-500/20 border-2 border-red-500'
              }`}
            >
              {event.outcome.success ? (
                <svg className="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
              ) : (
                <svg className="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                </svg>
              )}
            </div>

            {/* Event card */}
            <div className="flex-1 min-w-0">
              <button
                onClick={() => setExpandedEvent(expandedEvent === event.event_id ? null : event.event_id)}
                className="w-full text-left bg-slate-900 border border-slate-700 rounded-lg p-4 hover:border-slate-600 transition-colors"
              >
                <div className="flex items-start justify-between gap-4">
                  <div className="min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <span className={`text-sm font-medium ${
                        event.outcome.success ? 'text-green-400' : 'text-red-400'
                      }`}>
                        {event.event_type.replace(/_/g, ' ').toUpperCase()}
                      </span>
                      <span className="text-slate-500 text-sm">
                        {event.source_system}
                      </span>
                    </div>
                    <p className="text-white truncate">{event.summary}</p>
                    {event.agent_id && (
                      <p className="text-sm text-slate-400 mt-1">
                        Agent: <span className="font-mono">{event.agent_id}</span>
                      </p>
                    )}
                    {event.outcome.reason && (
                      <p className="text-sm text-red-400 mt-1">
                        {event.outcome.reason}
                      </p>
                    )}
                  </div>
                  <div className="text-right shrink-0">
                    <div className="text-sm text-slate-400">
                      {format(new Date(event.timestamp), 'HH:mm:ss')}
                    </div>
                    <div className="text-xs text-slate-500">
                      {format(new Date(event.timestamp), 'MMM d')}
                    </div>
                  </div>
                </div>

                {/* Expanded details */}
                {expandedEvent === event.event_id && (
                  <div className="mt-4 pt-4 border-t border-slate-700">
                    <pre className="text-xs text-slate-400 overflow-auto max-h-60 bg-slate-950 p-3 rounded">
                      {JSON.stringify(event.details, null, 2)}
                    </pre>
                  </div>
                )}
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
