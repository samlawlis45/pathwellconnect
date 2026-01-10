'use client';

import { createContext, useContext, useState, ReactNode } from 'react';

export interface Tenant {
  id: string;
  name: string;
  type: 'parent' | 'subsidiary';
  systems: string[];
}

export const TENANTS: Tenant[] = [
  {
    id: 'acme-company',
    name: 'ACME Company',
    type: 'parent',
    systems: ['All Systems'],
  },
  {
    id: 'acme-manufacturing',
    name: 'ACME Manufacturing',
    type: 'subsidiary',
    systems: ['SAP S/4HANA', 'Celonis iPaaS', 'Oracle TMS', 'Salesforce'],
  },
  {
    id: 'acme-distributing',
    name: 'ACME Distributing',
    type: 'subsidiary',
    systems: ['NetSuite', 'Boomi iPaaS', 'BluJay TMS', 'HubSpot'],
  },
];

interface TenantContextType {
  currentTenant: Tenant;
  setCurrentTenant: (tenant: Tenant) => void;
  tenants: Tenant[];
}

const TenantContext = createContext<TenantContextType | undefined>(undefined);

export function TenantProvider({ children }: { children: ReactNode }) {
  const [currentTenant, setCurrentTenant] = useState<Tenant>(TENANTS[0]);

  return (
    <TenantContext.Provider value={{ currentTenant, setCurrentTenant, tenants: TENANTS }}>
      {children}
    </TenantContext.Provider>
  );
}

export function useTenant() {
  const context = useContext(TenantContext);
  if (context === undefined) {
    throw new Error('useTenant must be used within a TenantProvider');
  }
  return context;
}
