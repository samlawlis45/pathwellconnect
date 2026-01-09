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
        <div className="min-h-screen">
          <header className="border-b border-slate-700 bg-slate-900/80 backdrop-blur-sm sticky top-0 z-50">
            <div className="max-w-7xl mx-auto px-4 py-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-8">
                  <a href="/" className="flex items-center gap-2">
                    <div className="w-8 h-8 rounded-lg bg-pathwell-blue flex items-center justify-center">
                      <svg className="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                      </svg>
                    </div>
                    <span className="text-xl font-semibold text-white">Pathwell Connect</span>
                  </a>
                  <nav className="flex items-center gap-6">
                    <a href="/" className="text-slate-300 hover:text-white transition-colors">Dashboard</a>
                    <a href="/traces" className="text-slate-300 hover:text-white transition-colors">Traces</a>
                    <a href="/lookup" className="text-slate-300 hover:text-white transition-colors">Lookup</a>
                  </nav>
                </div>
              </div>
            </div>
          </header>
          <main className="max-w-7xl mx-auto px-4 py-8">
            {children}
          </main>
        </div>
      </body>
    </html>
  );
}
