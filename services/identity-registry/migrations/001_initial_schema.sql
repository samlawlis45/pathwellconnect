-- Pathwell Connect Identity Registry Schema
-- Initial migration

-- Enterprises table
CREATE TABLE IF NOT EXISTS enterprises (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    enterprise_id VARCHAR(255) UNIQUE NOT NULL,
    public_key TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_enterprises_enterprise_id ON enterprises(enterprise_id);

-- Developers table
CREATE TABLE IF NOT EXISTS developers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    developer_id VARCHAR(255) UNIQUE NOT NULL,
    enterprise_id UUID REFERENCES enterprises(id),
    public_key TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_developers_developer_id ON developers(developer_id);
CREATE INDEX idx_developers_enterprise_id ON developers(enterprise_id);

-- Agents table
CREATE TABLE IF NOT EXISTS agents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agent_id VARCHAR(255) UNIQUE NOT NULL,
    developer_id UUID NOT NULL REFERENCES developers(id),
    enterprise_id UUID REFERENCES enterprises(id),
    public_key TEXT NOT NULL,
    certificate_chain TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_agents_agent_id ON agents(agent_id);
CREATE INDEX idx_agents_developer_id ON agents(developer_id);
CREATE INDEX idx_agents_enterprise_id ON agents(enterprise_id);
CREATE INDEX idx_agents_revoked ON agents(revoked_at) WHERE revoked_at IS NULL;

