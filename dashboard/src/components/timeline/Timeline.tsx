'use client';

import { useState } from 'react';
import { format } from 'date-fns';
import { Icon } from '@iconify/react';
import type { TimelineEvent } from '@/lib/types';

interface TimelineProps {
  events: TimelineEvent[];
}

export function Timeline({ events }: TimelineProps) {
  const [expandedEvent, setExpandedEvent] = useState<string | null>(null);

  if (events.length === 0) {
    return (
      <div className="text-center text-slate-400 py-8 flex flex-col items-center">
        <Icon icon="solar:inbox-outline" className="w-8 h-8 mb-2" />
        No events recorded yet
      </div>
    );
  }

  return (
    <div className="relative">
      {/* Vertical line */}
      <div className="absolute left-[23px] top-4 bottom-4 w-0.5 bg-slate-200" />

      <div className="space-y-4">
        {events.map((event) => (
          <div key={event.event_id} className="relative flex items-start gap-4">
            {/* Timeline node */}
            <div
              className={`relative z-10 w-12 h-12 rounded-full flex items-center justify-center shrink-0 ${
                event.outcome.success
                  ? 'bg-emerald-50 border-2 border-emerald-400'
                  : 'bg-red-50 border-2 border-red-400'
              }`}
            >
              {event.outcome.success ? (
                <Icon icon="solar:check-circle-bold" className="w-5 h-5 text-emerald-500" />
              ) : (
                <Icon icon="solar:close-circle-bold" className="w-5 h-5 text-red-500" />
              )}
            </div>

            {/* Event card */}
            <div className="flex-1 min-w-0">
              <button
                onClick={() => setExpandedEvent(expandedEvent === event.event_id ? null : event.event_id)}
                className="w-full text-left bg-white border border-slate-200 rounded-lg p-4 hover:border-slate-300 hover:shadow-sm transition-all"
              >
                <div className="flex items-start justify-between gap-4">
                  <div className="min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <span className={`text-sm font-medium ${
                        event.outcome.success ? 'text-emerald-600' : 'text-red-600'
                      }`}>
                        {event.event_type.replace(/_/g, ' ').toUpperCase()}
                      </span>
                      <span className="text-slate-400 text-sm">
                        {event.source_system}
                      </span>
                    </div>
                    <p className="text-slate-900">{event.summary}</p>
                    {event.agent_id && (
                      <p className="text-sm text-slate-500 mt-1">
                        Agent: <span className="font-mono text-slate-700">{event.agent_id}</span>
                      </p>
                    )}
                    {event.outcome.reason && (
                      <p className="text-sm text-red-500 mt-1 flex items-center gap-1">
                        <Icon icon="solar:danger-triangle-outline" className="w-4 h-4" />
                        {event.outcome.reason}
                      </p>
                    )}
                  </div>
                  <div className="text-right shrink-0">
                    <div className="text-sm text-slate-600 font-mono">
                      {format(new Date(event.timestamp), 'HH:mm:ss')}
                    </div>
                    <div className="text-xs text-slate-400">
                      {format(new Date(event.timestamp), 'MMM d')}
                    </div>
                  </div>
                </div>

                {/* Expanded details */}
                {expandedEvent === event.event_id && (
                  <div className="mt-4 pt-4 border-t border-slate-100">
                    <div className="flex items-center gap-2 text-xs text-slate-500 mb-2">
                      <Icon icon="solar:code-square-outline" className="w-4 h-4" />
                      Event Details
                    </div>
                    <pre className="text-xs text-slate-600 overflow-auto max-h-60 bg-slate-50 p-3 rounded-lg font-mono">
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
