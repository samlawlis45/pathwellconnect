-- Migration: Intelligent Ledger Support
-- Extends receipts system for full event storage, tracing, and querying

-- Traces table: Summary info per transaction trace
CREATE TABLE IF NOT EXISTS traces (
    trace_id UUID PRIMARY KEY,
    correlation_id VARCHAR(255),

    -- Trace lifecycle
    started_at TIMESTAMPTZ NOT NULL,
    last_event_at TIMESTAMPTZ NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',

    -- Summary counts
    event_count INTEGER NOT NULL DEFAULT 1,
    policy_deny_count INTEGER NOT NULL DEFAULT 0,

    -- First event context
    initiating_agent_id VARCHAR(255),
    initiating_developer_id UUID,
    enterprise_id VARCHAR(100),

    -- Metadata
    metadata JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_traces_correlation_id ON traces(correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_traces_status ON traces(status);
CREATE INDEX idx_traces_started_at ON traces(started_at DESC);
CREATE INDEX idx_traces_last_event_at ON traces(last_event_at DESC);
CREATE INDEX idx_traces_enterprise_id ON traces(enterprise_id) WHERE enterprise_id IS NOT NULL;
CREATE INDEX idx_traces_agent_id ON traces(initiating_agent_id) WHERE initiating_agent_id IS NOT NULL;

-- Receipt events table: Full receipt storage with all fields indexed
CREATE TABLE IF NOT EXISTS receipt_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    receipt_id UUID NOT NULL UNIQUE,
    trace_id UUID NOT NULL REFERENCES traces(trace_id),
    correlation_id VARCHAR(255),
    span_id UUID NOT NULL,
    parent_span_id UUID,

    -- Timing
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Event classification
    event_type VARCHAR(50) NOT NULL,
    event_source_system VARCHAR(100) NOT NULL,
    event_source_service VARCHAR(100) NOT NULL,
    event_source_version VARCHAR(50),

    -- Core data (denormalized for query performance)
    agent_id VARCHAR(255),
    developer_id UUID,
    enterprise_id UUID,

    -- Request info
    request_method VARCHAR(10),
    request_path TEXT,
    request_headers JSONB,
    request_body_hash VARCHAR(64),

    -- Results
    policy_allowed BOOLEAN,
    policy_version VARCHAR(50),
    policy_evaluation_ms INTEGER,
    identity_valid BOOLEAN,

    -- Flexible storage
    metadata JSONB,
    full_receipt JSONB NOT NULL,

    -- Chain integrity
    receipt_hash VARCHAR(64) NOT NULL,
    previous_receipt_hash VARCHAR(64),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Correlation index (primary query pattern)
CREATE INDEX idx_receipt_events_trace_id ON receipt_events(trace_id);
CREATE INDEX idx_receipt_events_correlation_id ON receipt_events(correlation_id) WHERE correlation_id IS NOT NULL;

-- Time-based queries
CREATE INDEX idx_receipt_events_timestamp ON receipt_events(timestamp DESC);
CREATE INDEX idx_receipt_events_trace_timestamp ON receipt_events(trace_id, timestamp);

-- Agent/Developer/Enterprise filtering
CREATE INDEX idx_receipt_events_agent_id ON receipt_events(agent_id);
CREATE INDEX idx_receipt_events_developer_id ON receipt_events(developer_id);
CREATE INDEX idx_receipt_events_enterprise_id ON receipt_events(enterprise_id);

-- Event type filtering
CREATE INDEX idx_receipt_events_event_type ON receipt_events(event_type);

-- Policy outcome queries
CREATE INDEX idx_receipt_events_policy_denied ON receipt_events(timestamp DESC) WHERE policy_allowed = false;

-- Hash chain verification
CREATE INDEX idx_receipt_events_hash ON receipt_events(receipt_hash);

-- External events table: Events from SAP, Salesforce, etc.
CREATE TABLE IF NOT EXISTS external_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL UNIQUE,
    trace_id UUID NOT NULL REFERENCES traces(trace_id),
    correlation_id VARCHAR(255),

    -- Event details
    event_type VARCHAR(100) NOT NULL,
    source_system VARCHAR(100) NOT NULL,
    source_id VARCHAR(255) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,

    -- Actor info
    actor_type VARCHAR(50),
    actor_id VARCHAR(255),
    actor_display_name VARCHAR(255),

    -- Payload
    payload JSONB NOT NULL,
    metadata JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_external_events_trace_id ON external_events(trace_id);
CREATE INDEX idx_external_events_correlation_id ON external_events(correlation_id) WHERE correlation_id IS NOT NULL;
CREATE INDEX idx_external_events_source ON external_events(source_system, source_id);
CREATE INDEX idx_external_events_timestamp ON external_events(timestamp DESC);
CREATE INDEX idx_external_events_event_type ON external_events(event_type);

-- Function to update trace summary on new events
CREATE OR REPLACE FUNCTION update_trace_on_event()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE traces
    SET
        event_count = event_count + 1,
        last_event_at = NEW.timestamp,
        policy_deny_count = policy_deny_count + CASE WHEN NEW.policy_allowed = false THEN 1 ELSE 0 END,
        updated_at = NOW()
    WHERE trace_id = NEW.trace_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to auto-update trace on receipt insert
CREATE TRIGGER trigger_update_trace_on_receipt
    AFTER INSERT ON receipt_events
    FOR EACH ROW
    EXECUTE FUNCTION update_trace_on_event();

-- Function to update trace on external event
CREATE OR REPLACE FUNCTION update_trace_on_external_event()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE traces
    SET
        event_count = event_count + 1,
        last_event_at = NEW.timestamp,
        updated_at = NOW()
    WHERE trace_id = NEW.trace_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for external events
CREATE TRIGGER trigger_update_trace_on_external
    AFTER INSERT ON external_events
    FOR EACH ROW
    EXECUTE FUNCTION update_trace_on_external_event();
