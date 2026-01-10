'use client';

import { ReactNode } from 'react';
import { TenantProvider } from '@/contexts/TenantContext';

export function Providers({ children }: { children: ReactNode }) {
  return <TenantProvider>{children}</TenantProvider>;
}
