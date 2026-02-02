-- ============================================================
-- Team Members reference table
-- ============================================================

CREATE TABLE team_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    role TEXT,
    department TEXT,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_team_members_active ON team_members(active);

CREATE TRIGGER trg_team_members_updated_at
    BEFORE UPDATE ON team_members
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================
-- Add responsible_id FK to machines
-- ============================================================

ALTER TABLE machines ADD COLUMN responsible_id UUID REFERENCES team_members(id) ON DELETE SET NULL;
CREATE INDEX idx_machines_responsible_id ON machines(responsible_id);
