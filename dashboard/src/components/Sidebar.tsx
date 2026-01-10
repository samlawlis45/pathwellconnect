'use client';

import { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Icon } from '@iconify/react';
import { useTenant, Tenant } from '@/contexts/TenantContext';

interface NavItem {
  href: string;
  label: string;
  icon: string;
}

const mainNavItems: NavItem[] = [
  { href: '/', label: 'Dashboard', icon: 'solar:home-2-outline' },
  { href: '/traces', label: 'Traces', icon: 'solar:timeline-down-outline' },
  { href: '/lookup', label: 'Lookup', icon: 'solar:magnifer-outline' },
];

const bottomNavItems: NavItem[] = [
  { href: '/settings', label: 'Settings', icon: 'solar:settings-outline' },
];

export function Sidebar() {
  const [isCollapsed, setIsCollapsed] = useState(false);
  const pathname = usePathname();

  const isActive = (href: string) => {
    if (href === '/') return pathname === '/';
    return pathname.startsWith(href);
  };

  return (
    <aside
      className={`flex flex-col bg-white border-r border-slate-200 transition-all duration-300 ${
        isCollapsed ? 'w-[72px]' : 'w-64'
      }`}
    >
      {/* Logo & Collapse Toggle */}
      <div className="h-16 flex items-center justify-between px-4 border-b border-slate-100">
        <Link href="/" className="flex items-center gap-3 overflow-hidden">
          <div className="w-9 h-9 rounded-full bg-gradient-to-br from-pathwell-500 to-pathwell-400 flex items-center justify-center shrink-0">
            <svg className="w-5 h-5 text-white" viewBox="0 0 24 24" fill="none">
              <path d="M4 12C4 7.58172 7.58172 4 12 4C16.4183 4 20 7.58172 20 12" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
              <path d="M12 4L19 16" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
              <path d="M12 4L12 20" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
            </svg>
          </div>
          {!isCollapsed && (
            <span className="font-semibold text-slate-900 whitespace-nowrap">Pathwell</span>
          )}
        </Link>
        <button
          onClick={() => setIsCollapsed(!isCollapsed)}
          className="p-1.5 rounded-lg hover:bg-slate-100 transition-colors text-slate-400 hover:text-slate-600"
        >
          <Icon
            icon={isCollapsed ? 'solar:alt-arrow-right-outline' : 'solar:alt-arrow-left-outline'}
            className="w-5 h-5"
          />
        </button>
      </div>

      {/* Tenant Selector */}
      <div className={`border-b border-slate-100 ${isCollapsed ? 'px-2 py-3' : 'px-3 py-4'}`}>
        <SidebarTenantSelector isCollapsed={isCollapsed} />
      </div>

      {/* Main Navigation */}
      <nav className="flex-1 px-3 py-4 space-y-1">
        {mainNavItems.map((item) => {
          const active = isActive(item.href);
          return (
            <Link
              key={item.href}
              href={item.href}
              className={`flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors ${
                active
                  ? 'bg-pathwell-50 text-pathwell-700'
                  : 'text-slate-600 hover:bg-slate-50 hover:text-slate-900'
              }`}
              title={isCollapsed ? item.label : undefined}
            >
              <Icon icon={item.icon} className={`w-5 h-5 shrink-0 ${active ? 'text-pathwell-600' : ''}`} />
              {!isCollapsed && (
                <span className="font-medium text-sm">{item.label}</span>
              )}
            </Link>
          );
        })}
      </nav>

      {/* Bottom Navigation */}
      <div className="px-3 py-4 border-t border-slate-100 space-y-1">
        {bottomNavItems.map((item) => {
          const active = isActive(item.href);
          return (
            <Link
              key={item.href}
              href={item.href}
              className={`flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors ${
                active
                  ? 'bg-pathwell-50 text-pathwell-700'
                  : 'text-slate-600 hover:bg-slate-50 hover:text-slate-900'
              }`}
              title={isCollapsed ? item.label : undefined}
            >
              <Icon icon={item.icon} className={`w-5 h-5 shrink-0 ${active ? 'text-pathwell-600' : ''}`} />
              {!isCollapsed && (
                <span className="font-medium text-sm">{item.label}</span>
              )}
            </Link>
          );
        })}

        {/* User Profile Placeholder */}
        <div className={`flex items-center gap-3 px-3 py-2 mt-2 ${isCollapsed ? 'justify-center' : ''}`}>
          <div className="w-8 h-8 rounded-full bg-slate-200 flex items-center justify-center shrink-0">
            <Icon icon="solar:user-outline" className="w-4 h-4 text-slate-500" />
          </div>
          {!isCollapsed && (
            <div className="overflow-hidden">
              <div className="text-sm font-medium text-slate-700 truncate">Guest User</div>
              <div className="text-xs text-slate-400 truncate">Sign in to save</div>
            </div>
          )}
        </div>
      </div>
    </aside>
  );
}

function SidebarTenantSelector({ isCollapsed }: { isCollapsed: boolean }) {
  const { currentTenant, setCurrentTenant, tenants } = useTenant();
  const [isOpen, setIsOpen] = useState(false);

  const handleSelect = (tenant: Tenant) => {
    setCurrentTenant(tenant);
    setIsOpen(false);
  };

  if (isCollapsed) {
    return (
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-center p-2 rounded-lg hover:bg-slate-50 transition-colors relative"
        title={currentTenant.name}
      >
        <div className={`w-8 h-8 rounded-lg flex items-center justify-center ${
          currentTenant.type === 'parent'
            ? 'bg-pathwell-100 text-pathwell-600'
            : 'bg-emerald-50 text-emerald-600'
        }`}>
          <Icon
            icon={currentTenant.type === 'parent' ? 'solar:buildings-outline' : 'solar:building-outline'}
            className="w-4 h-4"
          />
        </div>
        {isOpen && (
          <TenantDropdown
            tenants={tenants}
            currentTenant={currentTenant}
            onSelect={handleSelect}
            onClose={() => setIsOpen(false)}
            position="right"
          />
        )}
      </button>
    );
  }

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg border border-slate-200 hover:bg-slate-50 transition-colors"
      >
        <div className={`w-8 h-8 rounded-lg flex items-center justify-center shrink-0 ${
          currentTenant.type === 'parent'
            ? 'bg-pathwell-100 text-pathwell-600'
            : 'bg-emerald-50 text-emerald-600'
        }`}>
          <Icon
            icon={currentTenant.type === 'parent' ? 'solar:buildings-outline' : 'solar:building-outline'}
            className="w-4 h-4"
          />
        </div>
        <div className="flex-1 text-left overflow-hidden">
          <div className="text-sm font-medium text-slate-900 truncate">{currentTenant.name}</div>
          <div className="text-xs text-slate-500">
            {currentTenant.type === 'parent' ? 'Parent Company' : 'Business Unit'}
          </div>
        </div>
        <Icon
          icon="solar:alt-arrow-down-outline"
          className={`w-4 h-4 text-slate-400 shrink-0 transition-transform ${isOpen ? 'rotate-180' : ''}`}
        />
      </button>
      {isOpen && (
        <TenantDropdown
          tenants={tenants}
          currentTenant={currentTenant}
          onSelect={handleSelect}
          onClose={() => setIsOpen(false)}
          position="below"
        />
      )}
    </div>
  );
}

function TenantDropdown({
  tenants,
  currentTenant,
  onSelect,
  onClose,
  position,
}: {
  tenants: Tenant[];
  currentTenant: Tenant;
  onSelect: (tenant: Tenant) => void;
  onClose: () => void;
  position: 'below' | 'right';
}) {
  return (
    <>
      {/* Backdrop */}
      <div className="fixed inset-0 z-40" onClick={onClose} />

      {/* Dropdown */}
      <div
        className={`absolute z-50 w-72 bg-white border border-slate-200 rounded-xl shadow-lg overflow-hidden ${
          position === 'right' ? 'left-full ml-2 top-0' : 'left-0 mt-2 top-full'
        }`}
      >
        <div className="px-3 py-2 bg-slate-50 border-b border-slate-100">
          <span className="text-xs font-medium text-slate-500 uppercase tracking-wide">Select Organization</span>
        </div>
        <div className="py-1 max-h-80 overflow-y-auto">
          {tenants.map((tenant) => (
            <button
              key={tenant.id}
              onClick={() => onSelect(tenant)}
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
    </>
  );
}
