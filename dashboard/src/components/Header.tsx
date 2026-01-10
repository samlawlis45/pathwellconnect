'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { TenantSelector } from './TenantSelector';

export function Header() {
  const pathname = usePathname();

  const navItems = [
    { href: '/', label: 'Dashboard' },
    { href: '/traces', label: 'Traces' },
    { href: '/lookup', label: 'Lookup' },
  ];

  return (
    <header className="border-b border-slate-200 bg-white sticky top-0 z-50">
      <div className="max-w-7xl mx-auto px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-10">
            <Link href="/" className="flex items-center gap-3">
              <div className="w-9 h-9 rounded-full bg-gradient-to-br from-pathwell-500 to-pathwell-400 flex items-center justify-center">
                <svg className="w-5 h-5 text-white" viewBox="0 0 24 24" fill="none">
                  <path d="M4 12C4 7.58172 7.58172 4 12 4C16.4183 4 20 7.58172 20 12" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                  <path d="M12 4L19 16" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                  <path d="M12 4L12 20" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                </svg>
              </div>
              <span className="text-lg font-semibold text-slate-900">Pathwell Connect</span>
            </Link>
            <nav className="flex items-center gap-1">
              {navItems.map((item) => {
                const isActive = pathname === item.href ||
                  (item.href !== '/' && pathname.startsWith(item.href));
                return (
                  <Link
                    key={item.href}
                    href={item.href}
                    className={`px-4 py-2 text-sm font-medium rounded-lg transition-colors ${
                      isActive
                        ? 'text-pathwell-600 bg-pathwell-50'
                        : 'text-slate-600 hover:text-pathwell-600 hover:bg-slate-50'
                    }`}
                  >
                    {item.label}
                  </Link>
                );
              })}
            </nav>
          </div>
          <TenantSelector />
        </div>
      </div>
    </header>
  );
}
