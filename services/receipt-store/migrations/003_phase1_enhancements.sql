-- Migration 003: Phase 1 MTCA Enhancements for Receipt Store
-- Adds enhanced attribution tracking and trust metadata

-- ========================================
-- PART A: Enhanced Attribution in Receipts
-- ========================================

-- Add attribution and trust columns to receipt_events
ALTER TABLE receipt_events
ADD COLUMN IF NOT EXISTS tenant_id UUID,
ADD COLUMN IF NOT EXISTS tenant_hierarchy_path TEXT[],
ADD COLUMN IF NOT EXISTS trust_score_at_event DECIMAL(5,4),
ADD COLUMN IF NOT EXISTS trust_dimensions_at_event JSONB,
ADD COLUMN IF NOT EXISTS trust_check_result VARCHAR(50),
ADD COLUMN IF NOT EXISTS attribution JSONB DEFAULT '{}';

-- Add to traces table for summary view
ALTER TABLE traces
ADD COLUMN IF NOT EXISTS tenant_id UUID,
ADD COLUMN IF NOT EXISTS tenant_hierarchy_path TEXT[],
ADD COLUMN IF NOT EXISTS min_trust_score DECIMAL(5,4),
ADD COLUMN IF NOT EXISTS avg_trust_score DECIMAL(5,4),
ADD COLUMN IF NOT EXISTS trust_violations INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS attribution_summary JSONB DEFAULT '{}';

-- Indexes for new columns
CREATE INDEX IF NOT EXISTS idx_receipt_events_tenant ON receipt_events(tenant_id);
CREATE INDEX IF NOT EXISTS idx_receipt_events_hierarchy ON receipt_events USING GIN(tenant_hierarchy_path);
CREATE INDEX IF NOT EXISTS idx_receipt_events_trust ON receipt_events(trust_score_at_event);
CREATE INDEX IF NOT EXISTS idx_receipt_events_trust_check ON receipt_events(trust_check_result);
CREATE INDEX IF NOT EXISTS idx_receipt_events_attribution ON receipt_events USING GIN(attribution);

CREATE INDEX IF NOT EXISTS idx_traces_tenant ON traces(tenant_id);
CREATE INDEX IF NOT EXISTS idx_traces_hierarchy ON traces USING GIN(tenant_hierarchy_path);
CREATE INDEX IF NOT EXISTS idx_traces_trust ON traces(min_trust_score);
CREATE INDEX IF NOT EXISTS idx_traces_trust_violations ON traces(trust_violations) WHERE trust_violations > 0;

-- ========================================
-- PART B: Trust Events Table
-- ========================================

CREATE TABLE IF NOT EXISTS trust_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trace_id UUID NOT NULL REFERENCES traces(trace_id),
    receipt_event_id UUID REFERENCES receipt_events(id),

    -- Trust context
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,

    -- Trust snapshot at event time
    trust_score DECIMAL(5,4) NOT NULL,
    trust_dimensions JSONB NOT NULL,
    threshold_applied DECIMAL(5,4),

    -- Event outcome
    event_type VARCHAR(100) NOT NULL,
    outcome VARCHAR(50) NOT NULL,
    reason TEXT,

    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB
);

CREATE INDEX idx_trust_events_trace ON trust_events(trace_id);
CREATE INDEX idx_trust_events_entity ON trust_events(entity_type, entity_id);
CREATE INDEX idx_trust_events_outcome ON trust_events(outcome);
CREATE INDEX idx_trust_events_timestamp ON trust_events(timestamp DESC);
CREATE INDEX idx_trust_events_type ON trust_events(event_type);

-- ========================================
-- PART C: Attribution Lineage Table
-- ========================================

CREATE TABLE IF NOT EXISTS attribution_lineage (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trace_id UUID REFERENCES traces(trace_id),

    -- Content identification
    content_type VARCHAR(100) NOT NULL,
    content_id VARCHAR(255) NOT NULL,
    content_version VARCHAR(50) NOT NULL,
    content_hash VARCHAR(64) NOT NULL,

    -- Lineage (TRUST.PROVENANCE)
    parent_content_id VARCHAR(255),
    parent_version VARCHAR(50),
    lineage_depth INTEGER NOT NULL DEFAULT 0,

    -- Attribution (AUTH.OBJ)
    creator_id UUID NOT NULL,
    publisher_id UUID,
    attribution_protocol_uri TEXT,
    licensing_terms JSONB,

    -- Revenue tracking (for Phase 2)
    revenue_token VARCHAR(255),
    royalty_distribution_map JSONB,

    -- Audit
    audit_visibility_scope VARCHAR(50) NOT NULL DEFAULT 'tenant',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_content_version UNIQUE (content_type, content_id, content_version)
);

CREATE INDEX idx_attribution_lineage_content ON attribution_lineage(content_type, content_id);
CREATE INDEX idx_attribution_lineage_creator ON attribution_lineage(creator_id);
CREATE INDEX idx_attribution_lineage_trace ON attribution_lineage(trace_id);
CREATE INDEX idx_attribution_lineage_parent ON attribution_lineage(parent_content_id, parent_version);
CREATE INDEX idx_attribution_lineage_revenue ON attribution_lineage(revenue_token) WHERE revenue_token IS NOT NULL;

-- ========================================
-- PART D: Update Functions
-- ========================================

-- Update trace summary with trust metrics when receipt event is inserted
CREATE OR REPLACE FUNCTION update_trace_trust_metrics()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.trust_score_at_event IS NOT NULL THEN
        UPDATE traces
        SET
            min_trust_score = LEAST(COALESCE(min_trust_score, 1.0), NEW.trust_score_at_event),
            avg_trust_score = (
                SELECT AVG(trust_score_at_event)
                FROM receipt_events
                WHERE trace_id = NEW.trace_id AND trust_score_at_event IS NOT NULL
            ),
            trust_violations = trust_violations + CASE
                WHEN NEW.trust_check_result = 'blocked' THEN 1
                ELSE 0
            END,
            updated_at = NOW()
        WHERE trace_id = NEW.trace_id;
    END IF;

    -- Update tenant info on trace if not set
    IF NEW.tenant_id IS NOT NULL THEN
        UPDATE traces
        SET
            tenant_id = COALESCE(tenant_id, NEW.tenant_id),
            tenant_hierarchy_path = COALESCE(tenant_hierarchy_path, NEW.tenant_hierarchy_path)
        WHERE trace_id = NEW.trace_id AND tenant_id IS NULL;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Drop existing trigger if it exists
DROP TRIGGER IF EXISTS trigger_update_trace_trust ON receipt_events;

CREATE TRIGGER trigger_update_trace_trust
    AFTER INSERT ON receipt_events
    FOR EACH ROW
    EXECUTE FUNCTION update_trace_trust_metrics();
