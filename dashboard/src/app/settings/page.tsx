'use client';

import { Icon } from '@iconify/react';
import { useTenant } from '@/contexts/TenantContext';

export default function SettingsPage() {
  const { currentTenant } = useTenant();

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-2xl font-semibold text-slate-900">Settings</h1>
        <p className="text-slate-500 mt-1">Manage your account and organization settings</p>
      </div>

      {/* Settings Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Profile Card */}
        <div className="bg-white border border-slate-200 rounded-xl p-6">
          <div className="flex items-center gap-4 mb-4">
            <div className="w-12 h-12 rounded-xl bg-slate-100 flex items-center justify-center">
              <Icon icon="solar:user-outline" className="w-6 h-6 text-slate-500" />
            </div>
            <div>
              <h2 className="font-semibold text-slate-900">Profile</h2>
              <p className="text-sm text-slate-500">Manage your personal information</p>
            </div>
          </div>
          <div className="p-4 bg-slate-50 rounded-lg border border-dashed border-slate-200 text-center">
            <Icon icon="solar:lock-outline" className="w-8 h-8 text-slate-300 mx-auto mb-2" />
            <p className="text-sm text-slate-400">Sign in to manage your profile</p>
          </div>
        </div>

        {/* Security Card */}
        <div className="bg-white border border-slate-200 rounded-xl p-6">
          <div className="flex items-center gap-4 mb-4">
            <div className="w-12 h-12 rounded-xl bg-slate-100 flex items-center justify-center">
              <Icon icon="solar:shield-check-outline" className="w-6 h-6 text-slate-500" />
            </div>
            <div>
              <h2 className="font-semibold text-slate-900">Security</h2>
              <p className="text-sm text-slate-500">Password and two-factor authentication</p>
            </div>
          </div>
          <div className="p-4 bg-slate-50 rounded-lg border border-dashed border-slate-200 text-center">
            <Icon icon="solar:lock-outline" className="w-8 h-8 text-slate-300 mx-auto mb-2" />
            <p className="text-sm text-slate-400">Sign in to manage security settings</p>
          </div>
        </div>

        {/* Organization Card */}
        <div className="bg-white border border-slate-200 rounded-xl p-6">
          <div className="flex items-center gap-4 mb-4">
            <div className="w-12 h-12 rounded-xl bg-pathwell-50 flex items-center justify-center">
              <Icon icon="solar:buildings-outline" className="w-6 h-6 text-pathwell-600" />
            </div>
            <div>
              <h2 className="font-semibold text-slate-900">Organization</h2>
              <p className="text-sm text-slate-500">Configure {currentTenant.name}</p>
            </div>
          </div>
          <div className="space-y-3">
            <div className="flex items-center justify-between py-2 border-b border-slate-100">
              <span className="text-sm text-slate-600">Type</span>
              <span className="text-sm font-medium text-slate-900 capitalize">{currentTenant.type}</span>
            </div>
            <div className="flex items-center justify-between py-2 border-b border-slate-100">
              <span className="text-sm text-slate-600">Systems</span>
              <span className="text-sm font-medium text-slate-900">{currentTenant.systems.length} connected</span>
            </div>
            <div className="flex items-center justify-between py-2">
              <span className="text-sm text-slate-600">ID</span>
              <code className="text-xs font-mono bg-slate-100 px-2 py-1 rounded text-slate-600">{currentTenant.id}</code>
            </div>
          </div>
        </div>

        {/* Team Card */}
        <div className="bg-white border border-slate-200 rounded-xl p-6">
          <div className="flex items-center gap-4 mb-4">
            <div className="w-12 h-12 rounded-xl bg-slate-100 flex items-center justify-center">
              <Icon icon="solar:users-group-rounded-outline" className="w-6 h-6 text-slate-500" />
            </div>
            <div>
              <h2 className="font-semibold text-slate-900">Team Members</h2>
              <p className="text-sm text-slate-500">Manage users and permissions</p>
            </div>
          </div>
          <div className="p-4 bg-slate-50 rounded-lg border border-dashed border-slate-200 text-center">
            <Icon icon="solar:lock-outline" className="w-8 h-8 text-slate-300 mx-auto mb-2" />
            <p className="text-sm text-slate-400">Sign in to manage team members</p>
          </div>
        </div>
      </div>
    </div>
  );
}
