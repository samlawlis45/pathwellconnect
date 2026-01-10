-- ACME Company Demo Data
-- Master tenant: ACME Company
-- Subtenants: ACME Manufacturing, ACME Distributing

-- Drop and recreate tables to ensure clean schema
DROP TABLE IF EXISTS external_events CASCADE;
DROP TABLE IF EXISTS receipt_events CASCADE;
DROP TABLE IF EXISTS traces CASCADE;

-- Traces table
CREATE TABLE traces (
    trace_id UUID PRIMARY KEY,
    correlation_id VARCHAR(255),
    started_at TIMESTAMPTZ NOT NULL,
    last_event_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    event_count INTEGER NOT NULL DEFAULT 1,
    policy_deny_count INTEGER NOT NULL DEFAULT 0,
    initiating_agent_id VARCHAR(255),
    initiating_developer_id UUID,
    enterprise_id VARCHAR(100),
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_traces_correlation_id ON traces(correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_traces_status ON traces(status);
CREATE INDEX idx_traces_enterprise_id ON traces(enterprise_id) WHERE enterprise_id IS NOT NULL;
CREATE INDEX idx_traces_last_event_at ON traces(last_event_at DESC);

-- External events table
CREATE TABLE external_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL UNIQUE,
    trace_id UUID NOT NULL REFERENCES traces(trace_id),
    correlation_id VARCHAR(255),
    event_type VARCHAR(100) NOT NULL,
    source_system VARCHAR(100) NOT NULL,
    source_id VARCHAR(255) NOT NULL DEFAULT '',
    timestamp TIMESTAMPTZ NOT NULL,
    actor_type VARCHAR(50),
    actor_id VARCHAR(255),
    actor_display_name VARCHAR(255),
    payload JSONB NOT NULL DEFAULT '{}',
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_external_events_trace_id ON external_events(trace_id);
CREATE INDEX idx_external_events_correlation_id ON external_events(correlation_id);
CREATE INDEX idx_external_events_timestamp ON external_events(timestamp DESC);

-- ============================================================
-- ACME MANUFACTURING - Uses SAP, Celonis, Oracle TMS, Salesforce
-- ============================================================

-- Trace 1: Purchase Order PO-MFG-2024-0847 (Complete flow)
INSERT INTO traces (trace_id, correlation_id, enterprise_id, initiating_agent_id, status, event_count, policy_deny_count, started_at, last_event_at)
VALUES (
  'a1000000-0000-0000-0000-000000000001',
  'PO-MFG-2024-0847',
  'acme-manufacturing',
  'salesforce-agent',
  'completed',
  8,
  0,
  NOW() - INTERVAL '4 hours',
  NOW() - INTERVAL '1 hour'
);

INSERT INTO external_events (event_id, trace_id, correlation_id, source_system, event_type, source_id, actor_id, payload, timestamp)
VALUES
  ('e1000001-0000-0000-0000-000000000001', 'a1000000-0000-0000-0000-000000000001', 'PO-MFG-2024-0847', 'Salesforce', 'opportunity_won', 'OPP-78234', 'salesforce-agent', '{"summary": "Opportunity closed - Industrial Motors contract", "opportunity_id": "OPP-78234", "account": "Midwest Industrial", "value": 125000, "rep": "Sarah Chen", "outcome": {"success": true}}', NOW() - INTERVAL '4 hours'),
  ('e1000002-0000-0000-0000-000000000001', 'a1000000-0000-0000-0000-000000000001', 'PO-MFG-2024-0847', 'Celonis iPaaS', 'workflow_triggered', 'WF-O2C-001', 'celonis-orchestrator', '{"summary": "Order-to-Cash workflow initiated", "workflow_id": "WF-O2C-001", "trigger": "opportunity_close", "outcome": {"success": true}}', NOW() - INTERVAL '3 hours 50 minutes'),
  ('e1000003-0000-0000-0000-000000000001', 'a1000000-0000-0000-0000-000000000001', 'PO-MFG-2024-0847', 'SAP S/4HANA', 'sales_order_created', '4500089234', 'sap-integration-agent', '{"summary": "Sales order created in SAP", "sap_order": "4500089234", "material": "MOTOR-IND-500HP", "quantity": 12, "plant": "US10", "outcome": {"success": true}}', NOW() - INTERVAL '3 hours 45 minutes'),
  ('e1000004-0000-0000-0000-000000000001', 'a1000000-0000-0000-0000-000000000001', 'PO-MFG-2024-0847', 'SAP S/4HANA', 'production_order_created', 'PR-2024-11234', 'sap-integration-agent', '{"summary": "Production order scheduled", "prod_order": "PR-2024-11234", "start_date": "2024-01-15", "end_date": "2024-01-22", "outcome": {"success": true}}', NOW() - INTERVAL '3 hours 30 minutes'),
  ('e1000005-0000-0000-0000-000000000001', 'a1000000-0000-0000-0000-000000000001', 'PO-MFG-2024-0847', 'SAP S/4HANA', 'goods_issue', 'DEL-8923456', 'sap-integration-agent', '{"summary": "Finished goods moved to shipping", "delivery": "DEL-8923456", "warehouse": "WH-CHICAGO", "outcome": {"success": true}}', NOW() - INTERVAL '2 hours'),
  ('e1000006-0000-0000-0000-000000000001', 'a1000000-0000-0000-0000-000000000001', 'PO-MFG-2024-0847', 'Oracle TMS', 'shipment_created', 'SHP-2024-78234', 'oracle-tms-agent', '{"summary": "Shipment planned with carrier selection", "shipment_id": "SHP-2024-78234", "carrier": "XPO Logistics", "mode": "LTL", "weight_lbs": 4800, "outcome": {"success": true}}', NOW() - INTERVAL '1 hour 45 minutes'),
  ('e1000007-0000-0000-0000-000000000001', 'a1000000-0000-0000-0000-000000000001', 'PO-MFG-2024-0847', 'Oracle TMS', 'carrier_tendered', 'TND-89234', 'oracle-tms-agent', '{"summary": "Load tendered to XPO Logistics", "tender_id": "TND-89234", "rate": 2340.00, "pickup_date": "2024-01-23", "outcome": {"success": true}}', NOW() - INTERVAL '1 hour 30 minutes'),
  ('e1000008-0000-0000-0000-000000000001', 'a1000000-0000-0000-0000-000000000001', 'PO-MFG-2024-0847', 'Celonis iPaaS', 'customer_notified', 'NOTIF-89234', 'celonis-orchestrator', '{"summary": "Shipment confirmation sent to customer", "notification_type": "shipment_confirmation", "recipient": "purchasing@midwestind.com", "outcome": {"success": true}}', NOW() - INTERVAL '1 hour');

-- Trace 2: Purchase Order PO-MFG-2024-0848 (Policy denial)
INSERT INTO traces (trace_id, correlation_id, enterprise_id, initiating_agent_id, status, event_count, policy_deny_count, started_at, last_event_at)
VALUES (
  'a1000000-0000-0000-0000-000000000002',
  'PO-MFG-2024-0848',
  'acme-manufacturing',
  'salesforce-agent',
  'failed',
  4,
  1,
  NOW() - INTERVAL '2 hours',
  NOW() - INTERVAL '1 hour 30 minutes'
);

INSERT INTO external_events (event_id, trace_id, correlation_id, source_system, event_type, source_id, actor_id, payload, timestamp)
VALUES
  ('e1000001-0000-0000-0000-000000000002', 'a1000000-0000-0000-0000-000000000002', 'PO-MFG-2024-0848', 'Salesforce', 'opportunity_won', 'OPP-78235', 'salesforce-agent', '{"summary": "Opportunity closed - Precision Parts order", "opportunity_id": "OPP-78235", "account": "Precision Parts Ltd", "value": 89000, "outcome": {"success": true}}', NOW() - INTERVAL '2 hours'),
  ('e1000002-0000-0000-0000-000000000002', 'a1000000-0000-0000-0000-000000000002', 'PO-MFG-2024-0848', 'Celonis iPaaS', 'workflow_triggered', 'WF-O2C-001', 'celonis-orchestrator', '{"summary": "Order-to-Cash workflow initiated", "workflow_id": "WF-O2C-001", "outcome": {"success": true}}', NOW() - INTERVAL '1 hour 55 minutes'),
  ('e1000003-0000-0000-0000-000000000002', 'a1000000-0000-0000-0000-000000000002', 'PO-MFG-2024-0848', 'SAP S/4HANA', 'credit_check', 'PREC-001', 'sap-integration-agent', '{"summary": "Customer credit check performed", "customer": "PREC-001", "credit_limit": 50000, "outstanding": 62000, "outcome": {"success": false, "reason": "Credit limit exceeded"}}', NOW() - INTERVAL '1 hour 50 minutes'),
  ('e1000004-0000-0000-0000-000000000002', 'a1000000-0000-0000-0000-000000000002', 'PO-MFG-2024-0848', 'Pathwell Policy', 'policy_evaluation', 'POL-CREDIT-001', 'policy-engine', '{"summary": "Order blocked - credit policy violation", "policy": "credit-limit-check", "action": "block_order", "requires_approval": true, "outcome": {"success": false, "reason": "Policy denied: customer over credit limit by $12,000"}}', NOW() - INTERVAL '1 hour 30 minutes');

-- Trace 3: Invoice INV-MFG-2024-1234 (Active)
INSERT INTO traces (trace_id, correlation_id, enterprise_id, initiating_agent_id, status, event_count, policy_deny_count, started_at, last_event_at)
VALUES (
  'a1000000-0000-0000-0000-000000000003',
  'INV-MFG-2024-1234',
  'acme-manufacturing',
  'sap-integration-agent',
  'active',
  3,
  0,
  NOW() - INTERVAL '30 minutes',
  NOW() - INTERVAL '5 minutes'
);

INSERT INTO external_events (event_id, trace_id, correlation_id, source_system, event_type, source_id, actor_id, payload, timestamp)
VALUES
  ('e1000001-0000-0000-0000-000000000003', 'a1000000-0000-0000-0000-000000000003', 'INV-MFG-2024-1234', 'SAP S/4HANA', 'invoice_created', '90001234', 'sap-integration-agent', '{"summary": "Invoice generated for delivered goods", "invoice_number": "90001234", "amount": 125000, "currency": "USD", "due_date": "2024-02-15", "outcome": {"success": true}}', NOW() - INTERVAL '30 minutes'),
  ('e1000002-0000-0000-0000-000000000003', 'a1000000-0000-0000-0000-000000000003', 'INV-MFG-2024-1234', 'Celonis iPaaS', 'document_transform', 'TX-EDI-810', 'celonis-orchestrator', '{"summary": "Invoice converted to EDI 810 format", "format": "X12-810", "trading_partner": "MIDWEST-001", "outcome": {"success": true}}', NOW() - INTERVAL '25 minutes'),
  ('e1000003-0000-0000-0000-000000000003', 'a1000000-0000-0000-0000-000000000003', 'INV-MFG-2024-1234', 'Celonis iPaaS', 'edi_transmitted', 'TX-2024-89234', 'celonis-orchestrator', '{"summary": "EDI invoice transmitted to customer", "transmission_id": "TX-2024-89234", "status": "acknowledged", "outcome": {"success": true}}', NOW() - INTERVAL '5 minutes');

-- ============================================================
-- ACME DISTRIBUTING - Uses NetSuite, Boomi, BluJay TMS, HubSpot
-- ============================================================

-- Trace 4: Sales Order SO-DIST-2024-5678 (Complete flow)
INSERT INTO traces (trace_id, correlation_id, enterprise_id, initiating_agent_id, status, event_count, policy_deny_count, started_at, last_event_at)
VALUES (
  'b2000000-0000-0000-0000-000000000001',
  'SO-DIST-2024-5678',
  'acme-distributing',
  'hubspot-agent',
  'completed',
  7,
  0,
  NOW() - INTERVAL '6 hours',
  NOW() - INTERVAL '2 hours'
);

INSERT INTO external_events (event_id, trace_id, correlation_id, source_system, event_type, source_id, actor_id, payload, timestamp)
VALUES
  ('e2000001-0000-0000-0000-000000000001', 'b2000000-0000-0000-0000-000000000001', 'SO-DIST-2024-5678', 'HubSpot', 'deal_closed', 'DEAL-45678', 'hubspot-agent', '{"summary": "Deal closed won - Office Supplies bulk order", "deal_id": "DEAL-45678", "company": "Metro Office Solutions", "amount": 34500, "outcome": {"success": true}}', NOW() - INTERVAL '6 hours'),
  ('e2000002-0000-0000-0000-000000000001', 'b2000000-0000-0000-0000-000000000001', 'SO-DIST-2024-5678', 'Boomi iPaaS', 'integration_started', 'PROC-D2O-001', 'boomi-orchestrator', '{"summary": "Deal-to-Order integration triggered", "process_id": "PROC-D2O-001", "source": "hubspot", "target": "netsuite", "outcome": {"success": true}}', NOW() - INTERVAL '5 hours 55 minutes'),
  ('e2000003-0000-0000-0000-000000000001', 'b2000000-0000-0000-0000-000000000001', 'SO-DIST-2024-5678', 'NetSuite', 'sales_order_created', 'SO-78234', 'netsuite-agent', '{"summary": "Sales order created in NetSuite", "order_id": "SO-78234", "items": 24, "warehouse": "DIST-ATLANTA", "outcome": {"success": true}}', NOW() - INTERVAL '5 hours 50 minutes'),
  ('e2000004-0000-0000-0000-000000000001', 'b2000000-0000-0000-0000-000000000001', 'SO-DIST-2024-5678', 'NetSuite', 'inventory_allocated', 'ALLOC-34567', 'netsuite-agent', '{"summary": "Inventory allocated from Atlanta DC", "allocation_id": "ALLOC-34567", "items_fulfilled": 24, "backorder": 0, "outcome": {"success": true}}', NOW() - INTERVAL '5 hours'),
  ('e2000005-0000-0000-0000-000000000001', 'b2000000-0000-0000-0000-000000000001', 'SO-DIST-2024-5678', 'BluJay TMS', 'shipment_optimized', 'RT-2024-1234', 'blujay-agent', '{"summary": "Multi-stop route optimized", "route_id": "RT-2024-1234", "stops": 3, "total_miles": 487, "carrier": "FedEx Freight", "outcome": {"success": true}}', NOW() - INTERVAL '4 hours'),
  ('e2000006-0000-0000-0000-000000000001', 'b2000000-0000-0000-0000-000000000001', 'SO-DIST-2024-5678', 'BluJay TMS', 'pickup_scheduled', 'PU-892345', 'blujay-agent', '{"summary": "Carrier pickup confirmed", "pickup_number": "PU-892345", "pickup_date": "2024-01-20", "window": "08:00-12:00", "outcome": {"success": true}}', NOW() - INTERVAL '3 hours'),
  ('e2000007-0000-0000-0000-000000000001', 'b2000000-0000-0000-0000-000000000001', 'SO-DIST-2024-5678', 'Boomi iPaaS', 'tracking_synced', 'SYNC-78234', 'boomi-orchestrator', '{"summary": "Tracking info synced to HubSpot timeline", "sync_id": "SYNC-78234", "tracking_number": "794644790199", "outcome": {"success": true}}', NOW() - INTERVAL '2 hours');

-- Trace 5: Return RMA-DIST-2024-0089 (Active)
INSERT INTO traces (trace_id, correlation_id, enterprise_id, initiating_agent_id, status, event_count, policy_deny_count, started_at, last_event_at)
VALUES (
  'b2000000-0000-0000-0000-000000000002',
  'RMA-DIST-2024-0089',
  'acme-distributing',
  'netsuite-agent',
  'active',
  4,
  0,
  NOW() - INTERVAL '1 hour',
  NOW() - INTERVAL '10 minutes'
);

INSERT INTO external_events (event_id, trace_id, correlation_id, source_system, event_type, source_id, actor_id, payload, timestamp)
VALUES
  ('e2000001-0000-0000-0000-000000000002', 'b2000000-0000-0000-0000-000000000002', 'RMA-DIST-2024-0089', 'NetSuite', 'rma_created', 'RMA-0089', 'netsuite-agent', '{"summary": "Return authorization created", "rma_number": "RMA-0089", "original_order": "SO-77234", "reason": "Damaged in transit", "items": 3, "outcome": {"success": true}}', NOW() - INTERVAL '1 hour'),
  ('e2000002-0000-0000-0000-000000000002', 'b2000000-0000-0000-0000-000000000002', 'RMA-DIST-2024-0089', 'Boomi iPaaS', 'claim_initiated', 'CLM-2024-0234', 'boomi-orchestrator', '{"summary": "Carrier damage claim process started", "claim_id": "CLM-2024-0234", "carrier": "FedEx Freight", "declared_value": 1250, "outcome": {"success": true}}', NOW() - INTERVAL '50 minutes'),
  ('e2000003-0000-0000-0000-000000000002', 'b2000000-0000-0000-0000-000000000002', 'RMA-DIST-2024-0089', 'BluJay TMS', 'return_label_generated', '794644790288', 'blujay-agent', '{"summary": "Prepaid return label created", "tracking": "794644790288", "service": "FedEx Ground", "outcome": {"success": true}}', NOW() - INTERVAL '40 minutes'),
  ('e2000004-0000-0000-0000-000000000002', 'b2000000-0000-0000-0000-000000000002', 'RMA-DIST-2024-0089', 'HubSpot', 'customer_notified', 'EM-2024-78234', 'hubspot-agent', '{"summary": "Return instructions sent to customer", "email_id": "EM-2024-78234", "recipient": "ops@metrooffice.com", "outcome": {"success": true}}', NOW() - INTERVAL '10 minutes');

-- Trace 6: Transfer Order TO-DIST-2024-0456 (Between warehouses)
INSERT INTO traces (trace_id, correlation_id, enterprise_id, initiating_agent_id, status, event_count, policy_deny_count, started_at, last_event_at)
VALUES (
  'b2000000-0000-0000-0000-000000000003',
  'TO-DIST-2024-0456',
  'acme-distributing',
  'netsuite-agent',
  'active',
  3,
  0,
  NOW() - INTERVAL '45 minutes',
  NOW() - INTERVAL '15 minutes'
);

INSERT INTO external_events (event_id, trace_id, correlation_id, source_system, event_type, source_id, actor_id, payload, timestamp)
VALUES
  ('e2000001-0000-0000-0000-000000000003', 'b2000000-0000-0000-0000-000000000003', 'TO-DIST-2024-0456', 'NetSuite', 'transfer_order_created', 'TO-0456', 'netsuite-agent', '{"summary": "Inter-warehouse transfer initiated", "from_warehouse": "DIST-ATLANTA", "to_warehouse": "DIST-DALLAS", "sku_count": 45, "total_units": 1200, "outcome": {"success": true}}', NOW() - INTERVAL '45 minutes'),
  ('e2000002-0000-0000-0000-000000000003', 'b2000000-0000-0000-0000-000000000003', 'TO-DIST-2024-0456', 'BluJay TMS', 'ltl_quote_received', 'QUOTE-2024-789', 'blujay-agent', '{"summary": "LTL quotes received from carriers", "quotes": [{"carrier": "Estes", "rate": 890}, {"carrier": "SAIA", "rate": 945}, {"carrier": "Old Dominion", "rate": 875}], "selected": "Old Dominion", "outcome": {"success": true}}', NOW() - INTERVAL '30 minutes'),
  ('e2000003-0000-0000-0000-000000000003', 'b2000000-0000-0000-0000-000000000003', 'TO-DIST-2024-0456', 'BluJay TMS', 'bol_generated', 'BOL-2024-56789', 'blujay-agent', '{"summary": "Bill of Lading generated", "bol_number": "BOL-2024-56789", "pro_number": "495-123456", "pieces": 12, "weight": 2400, "outcome": {"success": true}}', NOW() - INTERVAL '15 minutes');

-- ============================================================
-- ACME COMPANY (Parent) - Cross-BU visibility
-- ============================================================

-- Trace 7: Intercompany Transfer IC-ACME-2024-0012
INSERT INTO traces (trace_id, correlation_id, enterprise_id, initiating_agent_id, status, event_count, policy_deny_count, started_at, last_event_at)
VALUES (
  'c3000000-0000-0000-0000-000000000001',
  'IC-ACME-2024-0012',
  'acme-company',
  'sap-integration-agent',
  'completed',
  6,
  0,
  NOW() - INTERVAL '8 hours',
  NOW() - INTERVAL '3 hours'
);

INSERT INTO external_events (event_id, trace_id, correlation_id, source_system, event_type, source_id, actor_id, payload, timestamp)
VALUES
  ('e3000001-0000-0000-0000-000000000001', 'c3000000-0000-0000-0000-000000000001', 'IC-ACME-2024-0012', 'SAP S/4HANA', 'intercompany_po_created', 'IC-PO-78234', 'sap-integration-agent', '{"summary": "IC Purchase Order from Distributing to Manufacturing", "ic_po": "IC-PO-78234", "selling_company": "1000", "buying_company": "2000", "material": "MOTOR-IND-250HP", "qty": 50, "outcome": {"success": true}}', NOW() - INTERVAL '8 hours'),
  ('e3000002-0000-0000-0000-000000000001', 'c3000000-0000-0000-0000-000000000001', 'IC-ACME-2024-0012', 'SAP S/4HANA', 'intercompany_so_created', 'IC-SO-89234', 'sap-integration-agent', '{"summary": "IC Sales Order created in Manufacturing", "ic_so": "IC-SO-89234", "reference_po": "IC-PO-78234", "outcome": {"success": true}}', NOW() - INTERVAL '7 hours 50 minutes'),
  ('e3000003-0000-0000-0000-000000000001', 'c3000000-0000-0000-0000-000000000001', 'IC-ACME-2024-0012', 'Celonis iPaaS', 'ic_reconciliation', 'REC-2024-0234', 'celonis-orchestrator', '{"summary": "Intercompany balances reconciled", "reconciliation_id": "REC-2024-0234", "status": "balanced", "amount": 175000, "outcome": {"success": true}}', NOW() - INTERVAL '6 hours'),
  ('e3000004-0000-0000-0000-000000000001', 'c3000000-0000-0000-0000-000000000001', 'IC-ACME-2024-0012', 'Oracle TMS', 'ic_shipment_created', 'IC-SHP-2024-0567', 'oracle-tms-agent', '{"summary": "Intercompany shipment scheduled", "shipment_id": "IC-SHP-2024-0567", "origin": "PLANT-CHICAGO", "dest": "DIST-ATLANTA", "outcome": {"success": true}}', NOW() - INTERVAL '5 hours'),
  ('e3000005-0000-0000-0000-000000000001', 'c3000000-0000-0000-0000-000000000001', 'IC-ACME-2024-0012', 'NetSuite', 'inventory_received', 'IR-2024-34567', 'netsuite-agent', '{"summary": "Goods received at Distributing warehouse", "receipt_id": "IR-2024-34567", "warehouse": "DIST-ATLANTA", "qty_received": 50, "outcome": {"success": true}}', NOW() - INTERVAL '3 hours 30 minutes'),
  ('e3000006-0000-0000-0000-000000000001', 'c3000000-0000-0000-0000-000000000001', 'IC-ACME-2024-0012', 'Boomi iPaaS', 'ic_invoice_matched', '3WM-2024-7823', 'boomi-orchestrator', '{"summary": "Intercompany invoice 3-way matched", "match_id": "3WM-2024-7823", "po": "IC-PO-78234", "receipt": "IR-2024-34567", "invoice": "IC-INV-89234", "outcome": {"success": true}}', NOW() - INTERVAL '3 hours');

-- Trace 8: Compliance Audit AUDIT-ACME-2024-Q4
INSERT INTO traces (trace_id, correlation_id, enterprise_id, initiating_agent_id, status, event_count, policy_deny_count, started_at, last_event_at)
VALUES (
  'c3000000-0000-0000-0000-000000000002',
  'AUDIT-ACME-2024-Q4',
  'acme-company',
  'compliance-agent',
  'active',
  2,
  0,
  NOW() - INTERVAL '20 minutes',
  NOW() - INTERVAL '5 minutes'
);

INSERT INTO external_events (event_id, trace_id, correlation_id, source_system, event_type, source_id, actor_id, payload, timestamp)
VALUES
  ('e3000001-0000-0000-0000-000000000002', 'c3000000-0000-0000-0000-000000000002', 'AUDIT-ACME-2024-Q4', 'Pathwell Compliance', 'audit_initiated', 'AUDIT-Q4-2024', 'compliance-agent', '{"summary": "Q4 SOX compliance audit started", "audit_type": "SOX-404", "scope": ["acme-manufacturing", "acme-distributing"], "controls_count": 47, "outcome": {"success": true}}', NOW() - INTERVAL '20 minutes'),
  ('e3000002-0000-0000-0000-000000000002', 'c3000000-0000-0000-0000-000000000002', 'AUDIT-ACME-2024-Q4', 'Pathwell Compliance', 'evidence_collected', 'EVD-2024-001', 'compliance-agent', '{"summary": "Automated evidence collection in progress", "evidence_items": 234, "systems_queried": ["SAP", "NetSuite", "Oracle TMS", "BluJay"], "completion": 45, "outcome": {"success": true}}', NOW() - INTERVAL '5 minutes');
