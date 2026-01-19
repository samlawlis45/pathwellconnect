-- Migration 002: Phase 1 MTCA Enhancements
-- Adds tenant hierarchy, trust infrastructure, and attribution metadata

-- ========================================
-- PART A: Enhanced Tenant Hierarchy (TEN.*)
-- ========================================

-- Tenant types enum for classification
CREATE TYPE tenant_type AS ENUM ('platform', 'parent', 'child', 'instance');
CREATE TYPE tenant_relationship_type AS ENUM ('owns', 'governs', 'delegates', 'observes');

-- Tenants table (TEN.P and TEN.C unified model)
CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR(255) UNIQUE NOT NULL,
    tenant_type tenant_type NOT NULL DEFAULT 'child',
    display_name VARCHAR(500),

    -- Hierarchical relationships
    parent_tenant_id UUID REFERENCES tenants(id),
    root_tenant_id UUID REFERENCES tenants(id),
    hierarchy_depth INTEGER NOT NULL DEFAULT 0,
    hierarchy_path TEXT[],

    -- TEN.GOV - Governance configuration
    governance_config JSONB NOT NULL DEFAULT '{
        "policy_scope": "inherit",
        "visibility_rules": {},
        "monetization_enabled": false,
        "data_residency": null
    }',

    -- TEN.VIZ - Observability configuration
    visibility_config JSONB NOT NULL DEFAULT '{
        "cross_tenant_visibility": "none",
        "audit_level": "standard",
        "allowed_observers": []
    }',

    -- Metadata
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deactivated_at TIMESTAMPTZ
);

CREATE INDEX idx_tenants_tenant_id ON tenants(tenant_id);
CREATE INDEX idx_tenants_parent ON tenants(parent_tenant_id);
CREATE INDEX idx_tenants_root ON tenants(root_tenant_id);
CREATE INDEX idx_tenants_type ON tenants(tenant_type);
CREATE INDEX idx_tenants_hierarchy_path ON tenants USING GIN(hierarchy_path);
CREATE INDEX idx_tenants_active ON tenants(tenant_id) WHERE deactivated_at IS NULL;

-- Tenant relationships (TEN.GOV expansion)
CREATE TABLE IF NOT EXISTS tenant_relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_tenant_id UUID NOT NULL REFERENCES tenants(id),
    target_tenant_id UUID NOT NULL REFERENCES tenants(id),
    relationship_type tenant_relationship_type NOT NULL,

    -- Relationship configuration
    permissions JSONB NOT NULL DEFAULT '{}',
    constraints JSONB,

    -- Validity
    valid_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_tenant_relationship UNIQUE (source_tenant_id, target_tenant_id, relationship_type)
);

CREATE INDEX idx_tenant_rel_source ON tenant_relationships(source_tenant_id);
CREATE INDEX idx_tenant_rel_target ON tenant_relationships(target_tenant_id);
CREATE INDEX idx_tenant_rel_type ON tenant_relationships(relationship_type);

-- ========================================
-- PART B: TRUST.SCORE and TRUST.VAULT
-- ========================================

-- Trust scores table (TRUST.SCORE)
CREATE TABLE IF NOT EXISTS trust_scores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,

    -- Composite trust score (0.0 to 1.0)
    composite_score DECIMAL(5,4) NOT NULL DEFAULT 0.5000,
    confidence_level DECIMAL(5,4) NOT NULL DEFAULT 0.5000,

    -- Dimension breakdown
    dimension_scores JSONB NOT NULL DEFAULT '{
        "behavior": 0.5,
        "validation": 0.5,
        "provenance": 0.5,
        "alignment": 0.5,
        "reputation": 0.5
    }',

    -- Calculation metadata
    calculation_version VARCHAR(50) NOT NULL DEFAULT 'v1.0.0',
    last_calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    calculation_inputs JSONB,

    -- Thresholds (TRUST.ASSURE)
    minimum_threshold DECIMAL(5,4),
    threshold_action VARCHAR(50) DEFAULT 'warn',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT unique_entity_trust UNIQUE (entity_type, entity_id)
);

CREATE INDEX idx_trust_scores_entity ON trust_scores(entity_type, entity_id);
CREATE INDEX idx_trust_scores_composite ON trust_scores(composite_score);
CREATE INDEX idx_trust_scores_below_threshold ON trust_scores(entity_type)
    WHERE composite_score < minimum_threshold;

-- Trust score history for trend analysis
CREATE TABLE IF NOT EXISTS trust_score_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    trust_score_id UUID NOT NULL REFERENCES trust_scores(id),
    composite_score DECIMAL(5,4) NOT NULL,
    dimension_scores JSONB NOT NULL,
    change_reason VARCHAR(500),
    change_event_id UUID,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_trust_history_score ON trust_score_history(trust_score_id, recorded_at DESC);

-- Trust vault for cryptographic verification (TRUST.VAULT)
CREATE TABLE IF NOT EXISTS trust_vault_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,

    -- Cryptographic material
    public_key_hash VARCHAR(64) NOT NULL,
    key_algorithm VARCHAR(50) NOT NULL DEFAULT 'ECDSA-P256',
    key_purpose VARCHAR(50) NOT NULL DEFAULT 'signing',

    -- Certificate chain for X.509
    certificate_fingerprint VARCHAR(64),
    certificate_chain_hash VARCHAR(64),
    issuer_fingerprint VARCHAR(64),

    -- Verification status
    verification_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    last_verified_at TIMESTAMPTZ,
    verification_method VARCHAR(100),

    -- Validity
    valid_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    revocation_reason VARCHAR(500),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_trust_vault_entity ON trust_vault_entries(entity_type, entity_id);
CREATE INDEX idx_trust_vault_pubkey ON trust_vault_entries(public_key_hash);
CREATE INDEX idx_trust_vault_cert ON trust_vault_entries(certificate_fingerprint);
CREATE INDEX idx_trust_vault_valid ON trust_vault_entries(entity_type, entity_id)
    WHERE revoked_at IS NULL AND (valid_until IS NULL OR valid_until > NOW());

-- ========================================
-- PART C: Trust Risk Events (TRUST.RISK)
-- ========================================

CREATE TYPE risk_severity AS ENUM ('low', 'medium', 'high', 'critical');
CREATE TYPE risk_status AS ENUM ('open', 'investigating', 'mitigated', 'resolved', 'accepted');

CREATE TABLE IF NOT EXISTS trust_risk_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID NOT NULL,

    -- Risk classification
    risk_type VARCHAR(100) NOT NULL,
    severity risk_severity NOT NULL,
    status risk_status NOT NULL DEFAULT 'open',

    -- Risk details
    description TEXT NOT NULL,
    evidence JSONB,
    impact_assessment JSONB,

    -- Mitigation
    mitigation_actions JSONB,
    mitigated_at TIMESTAMPTZ,
    mitigated_by VARCHAR(255),

    -- Correlation
    related_event_ids UUID[],
    trace_id UUID,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);

CREATE INDEX idx_risk_events_entity ON trust_risk_events(entity_type, entity_id);
CREATE INDEX idx_risk_events_severity ON trust_risk_events(severity, status);
CREATE INDEX idx_risk_events_open ON trust_risk_events(status) WHERE status IN ('open', 'investigating');
CREATE INDEX idx_risk_events_trace ON trust_risk_events(trace_id) WHERE trace_id IS NOT NULL;

-- ========================================
-- PART D: AUTH.OBJ Attribution Metadata
-- ========================================

-- Add attribution columns to agents table
ALTER TABLE agents
ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id),
ADD COLUMN IF NOT EXISTS attribution JSONB NOT NULL DEFAULT '{}',
ADD COLUMN IF NOT EXISTS trust_score_id UUID REFERENCES trust_scores(id),
ADD COLUMN IF NOT EXISTS metadata JSONB;

-- Add tenant reference to developers
ALTER TABLE developers
ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id),
ADD COLUMN IF NOT EXISTS trust_score_id UUID REFERENCES trust_scores(id),
ADD COLUMN IF NOT EXISTS metadata JSONB;

-- Add tenant reference to enterprises
ALTER TABLE enterprises
ADD COLUMN IF NOT EXISTS tenant_id UUID REFERENCES tenants(id),
ADD COLUMN IF NOT EXISTS trust_score_id UUID REFERENCES trust_scores(id),
ADD COLUMN IF NOT EXISTS metadata JSONB;

-- Indexes for new columns
CREATE INDEX IF NOT EXISTS idx_agents_tenant ON agents(tenant_id);
CREATE INDEX IF NOT EXISTS idx_agents_trust_score ON agents(trust_score_id);
CREATE INDEX IF NOT EXISTS idx_agents_attribution ON agents USING GIN(attribution);
CREATE INDEX IF NOT EXISTS idx_developers_tenant ON developers(tenant_id);
CREATE INDEX IF NOT EXISTS idx_enterprises_tenant ON enterprises(tenant_id);

-- ========================================
-- PART E: Helper Functions
-- ========================================

-- Trigger to maintain hierarchy path
CREATE OR REPLACE FUNCTION update_tenant_hierarchy()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_tenant_id IS NULL THEN
        NEW.hierarchy_depth := 0;
        NEW.hierarchy_path := ARRAY[NEW.tenant_id];
        NEW.root_tenant_id := NEW.id;
    ELSE
        SELECT
            hierarchy_depth + 1,
            hierarchy_path || NEW.tenant_id,
            COALESCE(root_tenant_id, id)
        INTO NEW.hierarchy_depth, NEW.hierarchy_path, NEW.root_tenant_id
        FROM tenants
        WHERE id = NEW.parent_tenant_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_tenant_hierarchy
    BEFORE INSERT OR UPDATE OF parent_tenant_id ON tenants
    FOR EACH ROW
    EXECUTE FUNCTION update_tenant_hierarchy();

-- Function to update timestamps
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_tenants_updated_at
    BEFORE UPDATE ON tenants
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trigger_trust_scores_updated_at
    BEFORE UPDATE ON trust_scores
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trigger_trust_risk_events_updated_at
    BEFORE UPDATE ON trust_risk_events
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();
