import React from 'react';

const PathwellAgentGovernanceDiagram = () => {
  // SVG Icons to replace emojis
  const AgentIcon = () => (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
      <path d="M12 2a10 10 0 1 0 10 10H12V2z" />
      <path d="M12 12L2.5 10" />
      <path d="M12 12L14 21.5" />
      <circle cx="12" cy="12" r="2" fill="currentColor" fillOpacity="0.2" />
    </svg>
  );

  const ShieldIcon = () => (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
    </svg>
  );

  const LockIcon = () => (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <rect x="3" y="11" width="18" height="11" rx="2" ry="2" />
      <path d="M7 11V7a5 5 0 0 1 10 0v4" />
    </svg>
  );
  
  // Added a specific Coin/Settlement icon for the gold accent
  const CoinIcon = () => (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
      <circle cx="12" cy="12" r="10" />
      <path d="M12 6v12M16 10l-4-4-4 4" transform="rotate(180 12 12)" />
    </svg>
  );

  return (
    <div style={{
      minHeight: '100vh',
      background: '#0a1628', // Deep navy blue
      backgroundImage: 'radial-gradient(circle at 50% 0%, #172a45 0%, #0a1628 70%)',
      padding: '60px',
      fontFamily: "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
      color: '#e2e8f0',
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
    }}>
      
      {/* 1. HEADER SECTION */}
      <div style={{ textAlign: 'center', marginBottom: '50px', position: 'relative', zIndex: 10 }}>
        <h1 style={{
          fontSize: '32px',
          fontWeight: '600',
          background: 'linear-gradient(90deg, #fff 0%, #94a3b8 100%)',
          WebkitBackgroundClip: 'text',
          WebkitTextFillColor: 'transparent',
          margin: '0 0 12px 0',
          letterSpacing: '-0.5px',
        }}>
          Pathwell Connect™
        </h1>
        <p style={{
          fontSize: '14px',
          color: '#64748b',
          maxWidth: '600px',
          margin: '0 auto',
          letterSpacing: '0.2px'
        }}>
          The Governance Substrate for the AI Era
        </p>
      </div>

      {/* MAIN ARCHITECTURE CONTAINER */}
      <div style={{
        position: 'relative',
        width: '1000px',
        display: 'flex',
        flexDirection: 'column',
        gap: '40px',
      }}>
        
        {/* LAYER 0: LLMs (The Reasoning Layer) */}
        <div style={{ display: 'flex', justifyContent: 'center', gap: '24px', position: 'relative' }}>
          {['OpenAI', 'Anthropic', 'Google', 'Open Source'].map((llm, i) => (
            <div key={i} style={{
              background: 'linear-gradient(135deg, #1e1b4b 0%, #0f172a 100%)',
              border: '1px solid #6366f1',
              borderRadius: '8px',
              padding: '14px 24px',
              minWidth: '140px',
              textAlign: 'center',
              boxShadow: '0 4px 20px rgba(99, 102, 241, 0.15)',
            }}>
              <div style={{ fontSize: '12px', fontWeight: '600', color: '#a5b4fc' }}>{llm}</div>
            </div>
          ))}
        </div>

        {/* Connector lines from LLMs to Agents */}
        <div style={{ 
          display: 'flex', 
          justifyContent: 'center', 
          position: 'relative',
          height: '30px',
          marginTop: '-30px',
          marginBottom: '-30px'
        }}>
          <div style={{
            width: '60%',
            height: '2px',
            background: 'linear-gradient(90deg, transparent 0%, #6366f1 20%, #6366f1 80%, transparent 100%)',
            position: 'absolute',
            top: '50%'
          }} />
          <div style={{
            width: '2px',
            height: '30px',
            background: '#6366f1',
            position: 'absolute',
            top: '50%'
          }} />
        </div>

        {/* LAYER 1: THE AGENT SWARM (Requestors) */}
        <div style={{ display: 'flex', justifyContent: 'center', gap: '20px', position: 'relative' }}>
          {['Autonomous Sales', 'Supply Chain Agent', 'Code Generator', 'Finance Copilot'].map((name, i) => (
            <div key={i} style={{
              background: '#0f172a',
              border: '1px solid #334155',
              borderRadius: '6px',
              padding: '12px 20px',
              width: '180px',
              display: 'flex',
              alignItems: 'center',
              gap: '10px',
              boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.5)',
              zIndex: 2
            }}>
              <div style={{ color: '#4a90d9' }}><AgentIcon /></div>
              <div>
                <div style={{ fontSize: '11px', color: '#94a3b8', fontWeight: '500' }}>IDENTITY: {`0x${10 + i}F...`}</div>
                <div style={{ fontSize: '12px', fontWeight: '600', color: '#f1f5f9' }}>{name}</div>
              </div>
              {/* Connector Line pointing down */}
              <div style={{
                position: 'absolute',
                bottom: '-40px',
                left: `calc(16% + ${i * 200 + 30}px)`, 
                width: '2px',
                height: '40px',
                background: 'linear-gradient(180deg, #334155 0%, #4a90d9 100%)',
                zIndex: 1
              }} />
            </div>
          ))}
        </div>

        {/* LAYER 2: PATHWELL CONNECT (The Intercept Layer) */}
        <div style={{
          background: 'rgba(15, 23, 42, 0.9)',
          border: '1px solid #4a90d9',
          borderRadius: '16px',
          padding: '30px',
          position: 'relative',
          boxShadow: '0 0 100px rgba(74, 144, 217, 0.12)',
          display: 'grid',
          gridTemplateColumns: '250px 1fr 250px',
          gap: '30px',
          zIndex: 5
        }}>
          {/* Label Tag */}
          <div style={{
            position: 'absolute',
            top: '-12px',
            left: '50%',
            transform: 'translateX(-50%)',
            background: '#0a1628',
            border: '1px solid #4a90d9',
            color: '#4a90d9',
            fontSize: '10px',
            fontWeight: 'bold',
            padding: '4px 12px',
            borderRadius: '12px',
            textTransform: 'uppercase',
            letterSpacing: '1px'
          }}>
            Governance Substrate (Fail-Closed)
          </div>

          {/* LEFT: Ingress & Identity */}
          <div style={{ borderRight: '1px solid #334155', paddingRight: '20px', display: 'flex', flexDirection: 'column', justifyContent: 'center', gap: '15px' }}>
            <div style={{ fontSize: '11px', color: '#64748b', textTransform: 'uppercase', letterSpacing: '1px', fontWeight: '600' }}>Ingress Validation</div>
            
            <div style={{ background: '#1e293b', padding: '12px', borderRadius: '6px', borderLeft: '3px solid #3b82f6' }}>
              <div style={{ fontSize: '12px', color: '#e2e8f0', fontWeight: '600' }}>Identity Resolution</div>
              <div style={{ fontSize: '10px', color: '#94a3b8', marginTop: '4px' }}>Hardware Anchor Check</div>
            </div>
            
            <div style={{ background: '#1e293b', padding: '12px', borderRadius: '6px', borderLeft: '3px solid #3b82f6' }}>
              <div style={{ fontSize: '12px', color: '#e2e8f0', fontWeight: '600' }}>Context Graph</div>
              <div style={{ fontSize: '10px', color: '#94a3b8', marginTop: '4px' }}>Precedent Search</div>
            </div>
          </div>

          {/* CENTER: The Logic Core */}
          <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
            <div style={{ fontSize: '11px', color: '#4a90d9', textTransform: 'uppercase', letterSpacing: '1px', fontWeight: '600', marginBottom: '15px' }}>Policy Execution</div>
            
            {/* The Gate Visual */}
            <div style={{
              width: '100%',
              height: '100%',
              background: 'radial-gradient(circle at center, rgba(74,144,217,0.15) 0%, transparent 70%)',
              border: '1px dashed #475569',
              borderRadius: '8px',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              flexDirection: 'column',
              padding: '20px'
            }}>
              <div style={{ 
                background: '#0f172a', 
                border: '1px solid #4a90d9', 
                borderRadius: '50%', 
                width: '60px', 
                height: '60px', 
                display: 'flex', 
                alignItems: 'center', 
                justifyContent: 'center',
                color: '#4a90d9',
                marginBottom: '15px',
                boxShadow: '0 0 20px rgba(74, 144, 217, 0.3)'
              }}>
                <ShieldIcon />
              </div>
              
              <div style={{ display: 'flex', gap: '10px', width: '100%' }}>
                <div style={{ flex: 1, background: 'rgba(34, 197, 94, 0.1)', border: '1px solid rgba(34, 197, 94, 0.3)', borderRadius: '4px', padding: '10px', textAlign: 'center' }}>
                  <div style={{ fontSize: '10px', fontWeight: '700', color: '#22c55e' }}>AUTHORIZED</div>
                  <div style={{ fontSize: '9px', color: '#64748b' }}>Attribution Valid</div>
                </div>
                <div style={{ flex: 1, background: 'rgba(239, 68, 68, 0.1)', border: '1px solid rgba(239, 68, 68, 0.3)', borderRadius: '4px', padding: '10px', textAlign: 'center' }}>
                  <div style={{ fontSize: '10px', fontWeight: '700', color: '#ef4444' }}>BLOCKED</div>
                  <div style={{ fontSize: '9px', color: '#64748b' }}>No Token</div>
                </div>
              </div>
            </div>
          </div>

          {/* RIGHT: Egress & Receipts - PROVENANCE FOCUSED */}
          <div style={{ borderLeft: '1px solid #334155', paddingLeft: '20px', display: 'flex', flexDirection: 'column', justifyContent: 'center', gap: '15px' }}>
            <div style={{ fontSize: '11px', color: '#4a90d9', textTransform: 'uppercase', letterSpacing: '1px', fontWeight: '600' }}>Provenance</div>
            
            <div style={{ background: '#1e293b', padding: '12px', borderRadius: '6px', borderLeft: '3px solid #4a90d9' }}>
              <div style={{ fontSize: '12px', color: '#e2e8f0', fontWeight: '600' }}>Immutable Receipt</div>
              <div style={{ fontSize: '10px', color: '#94a3b8', marginTop: '4px' }}>Hash-Chained Log</div>
            </div>
            
            <div style={{ background: '#1e293b', padding: '12px', borderRadius: '6px', borderLeft: '3px solid #4a90d9' }}>
              <div style={{ fontSize: '12px', color: '#e2e8f0', fontWeight: '600' }}>Decision Trace</div>
              <div style={{ fontSize: '10px', color: '#94a3b8', marginTop: '4px' }}>Replayable Audit</div>
            </div>
          </div>
        </div>

        {/* LAYER 3: INFRASTRUCTURE (The Resources) */}
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '16px', position: 'relative', marginTop: '0px' }}>
          
          {/* Vertical Connectors from Pathwell to Infra */}
           <div style={{ position: 'absolute', top: '-40px', left: '12.5%', width: '2px', height: '40px', background: '#334155' }} />
           <div style={{ position: 'absolute', top: '-40px', left: '37.5%', width: '2px', height: '40px', background: '#334155' }} />
           <div style={{ position: 'absolute', top: '-40px', left: '62.5%', width: '2px', height: '40px', background: '#334155' }} />
           <div style={{ position: 'absolute', top: '-40px', left: '87.5%', width: '2px', height: '40px', background: '#334155' }} />

          {/* Module 1 */}
          <div style={{ background: '#111827', border: '1px solid #1e293b', padding: '15px', borderRadius: '8px', textAlign: 'center' }}>
            <div style={{ fontSize: '11px', fontWeight: '600', color: '#3b82f6', marginBottom: '5px' }}>BUSINESS APPS</div>
            <div style={{ fontSize: '11px', color: '#64748b' }}>Salesforce / SAP / Workday</div>
          </div>
          
          {/* Module 2 */}
          <div style={{ background: '#111827', border: '1px solid #1e293b', padding: '15px', borderRadius: '8px', textAlign: 'center' }}>
            <div style={{ fontSize: '11px', fontWeight: '600', color: '#8b5cf6', marginBottom: '5px' }}>ORCHESTRATION</div>
            <div style={{ fontSize: '11px', color: '#64748b' }}>Temporal / Camunda</div>
          </div>

          {/* Module 3 */}
          <div style={{ background: '#111827', border: '1px solid #1e293b', padding: '15px', borderRadius: '8px', textAlign: 'center' }}>
            <div style={{ fontSize: '11px', fontWeight: '600', color: '#10b981', marginBottom: '5px' }}>ENTERPRISE DATA</div>
            <div style={{ fontSize: '11px', color: '#64748b' }}>Snowflake / Databricks</div>
          </div>

          {/* Module 4 */}
          <div style={{ background: '#111827', border: '1px solid #1e293b', padding: '15px', borderRadius: '8px', textAlign: 'center' }}>
            <div style={{ fontSize: '11px', fontWeight: '600', color: '#f59e0b', marginBottom: '5px' }}>iPaaS</div>
            <div style={{ fontSize: '11px', color: '#64748b' }}>MuleSoft / Boomi / Workato</div>
          </div>
        </div>

      </div>

      {/* FOOTER / PRINCIPLES */}
      <div style={{
        marginTop: '60px',
        display: 'flex',
        gap: '40px',
        borderTop: '1px solid #1e293b',
        paddingTop: '30px',
        width: '1000px',
        justifyContent: 'center'
      }}>
        {[
          { label: 'IDENTITY', text: 'Every agent anchored. No anonymous actors.' },
          { label: 'POLICY', text: 'Rules as code. Fail-closed by default.' },
          { label: 'GATE', text: 'If it can\'t block, it\'s not a gate.' },
          { label: 'RECEIPT', text: 'Every decision logged. Fully replayable.' }
        ].map((item, i) => (
          <div key={i} style={{ textAlign: 'left', maxWidth: '200px' }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '6px' }}>
              <div style={{color:'#4a90d9'}}><LockIcon/></div>
              <div style={{ fontSize: '11px', fontWeight: '700', color: '#4a90d9', letterSpacing: '1px' }}>{item.label}</div>
            </div>
            <div style={{ fontSize: '11px', color: '#64748b', lineHeight: '1.4' }}>{item.text}</div>
          </div>
        ))}
      </div>

    </div>
  );
};

export default PathwellAgentGovernanceDiagram;
