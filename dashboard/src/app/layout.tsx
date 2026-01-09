import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'Pathwell Connect - Intelligent Ledger',
  description: 'Transaction lineage explorer for AI agent governance',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="min-h-screen bg-white">
          <header className="border-b border-slate-200 bg-white sticky top-0 z-50">
            <div className="max-w-7xl mx-auto px-6 py-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-10">
                  <a href="/" className="flex items-center gap-3">
                    <div className="w-9 h-9 rounded-full bg-gradient-to-br from-pathwell-500 to-pathwell-400 flex items-center justify-center">
                      <svg className="w-5 h-5 text-white" viewBox="0 0 24 24" fill="none">
                        <path d="M4 12C4 7.58172 7.58172 4 12 4C16.4183 4 20 7.58172 20 12" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                        <path d="M12 4L19 16" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                        <path d="M12 4L12 20" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                      </svg>
                    </div>
                    <span className="text-lg font-semibold text-slate-900">Pathwell Connect</span>
                  </a>
                  <nav className="flex items-center gap-1">
                    <a href="/" className="px-4 py-2 text-sm font-medium text-slate-600 hover:text-pathwell-600 hover:bg-slate-50 rounded-lg transition-colors">
                      Dashboard
                    </a>
                    <a href="/traces" className="px-4 py-2 text-sm font-medium text-slate-600 hover:text-pathwell-600 hover:bg-slate-50 rounded-lg transition-colors">
                      Traces
                    </a>
                    <a href="/lookup" className="px-4 py-2 text-sm font-medium text-slate-600 hover:text-pathwell-600 hover:bg-slate-50 rounded-lg transition-colors">
                      Lookup
                    </a>
                  </nav>
                </div>
              </div>
            </div>
          </header>
          <main className="max-w-7xl mx-auto px-6 py-8">
            {children}
          </main>
        </div>
      </body>
    </html>
  );
}
