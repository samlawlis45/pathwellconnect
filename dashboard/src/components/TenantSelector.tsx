'use client';

import { useState, useRef, useEffect } from 'react';
import { Icon } from '@iconify/react';
import { useTenant, Tenant } from '@/contexts/TenantContext';

export function TenantSelector() {
  const { currentTenant, setCurrentTenant, tenants } = useTenant();
  const [isOpen, setIsOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    }
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const handleSelect = (tenant: Tenant) => {
    setCurrentTenant(tenant);
    setIsOpen(false);
  };

  return (
    <div className="relative" ref={dropdownRef}>
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-2 px-3 py-2 bg-slate-50 border border-slate-200 rounded-lg hover:bg-slate-100 transition-colors"
      >
        <div className="flex items-center gap-2">
          <div className={`w-2 h-2 rounded-full ${currentTenant.type === 'parent' ? 'bg-pathwell-500' : 'bg-emerald-500'}`} />
          <span className="text-sm font-medium text-slate-700">{currentTenant.name}</span>
        </div>
        <Icon
          icon="solar:alt-arrow-down-outline"
          className={`w-4 h-4 text-slate-400 transition-transform ${isOpen ? 'rotate-180' : ''}`}
        />
      </button>

      {isOpen && (
        <div className="absolute right-0 mt-2 w-72 bg-white border border-slate-200 rounded-xl shadow-lg z-50 overflow-hidden">
          <div className="px-3 py-2 bg-slate-50 border-b border-slate-100">
            <span className="text-xs font-medium text-slate-500 uppercase tracking-wide">Select Organization</span>
          </div>
          <div className="py-1">
            {tenants.map((tenant) => (
              <button
                key={tenant.id}
                onClick={() => handleSelect(tenant)}
                className={`w-full px-3 py-3 text-left hover:bg-slate-50 transition-colors ${
                  currentTenant.id === tenant.id ? 'bg-pathwell-50' : ''
                }`}
              >
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${
                      tenant.type === 'parent'
                        ? 'bg-pathwell-100 text-pathwell-600'
                        : 'bg-emerald-50 text-emerald-600'
                    }`}>
                      <Icon
                        icon={tenant.type === 'parent' ? 'solar:buildings-outline' : 'solar:building-outline'}
                        className="w-4 h-4"
                      />
                    </div>
                    <div>
                      <div className="text-sm font-medium text-slate-900">{tenant.name}</div>
                      <div className="text-xs text-slate-500">
                        {tenant.type === 'parent' ? 'Parent Company' : 'Business Unit'}
                      </div>
                    </div>
                  </div>
                  {currentTenant.id === tenant.id && (
                    <Icon icon="solar:check-circle-bold" className="w-5 h-5 text-pathwell-600" />
                  )}
                </div>
                <div className="mt-2 ml-11 flex flex-wrap gap-1">
                  {tenant.systems.slice(0, 3).map((system) => (
                    <span
                      key={system}
                      className="px-2 py-0.5 bg-slate-100 text-slate-600 text-xs rounded-md"
                    >
                      {system}
                    </span>
                  ))}
                  {tenant.systems.length > 3 && (
                    <span className="px-2 py-0.5 text-slate-400 text-xs">
                      +{tenant.systems.length - 3} more
                    </span>
                  )}
                </div>
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
