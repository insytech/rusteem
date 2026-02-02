-- ============================================================
-- Reference tables for machine classification
-- ============================================================

CREATE TABLE machine_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE manufacturers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    contact_name TEXT,
    contact_email TEXT,
    contact_phone TEXT,
    website TEXT,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    area TEXT NOT NULL,
    line TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(area, line)
);

CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    code TEXT UNIQUE,
    description TEXT,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================================================
-- Purchase/RFQ tracking per machine
-- ============================================================

CREATE TABLE purchase_rfqs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    machine_id UUID NOT NULL REFERENCES machines(id) ON DELETE CASCADE,
    rfq_number TEXT,
    purchase_order TEXT,
    tooling_agreement BOOLEAN NOT NULL DEFAULT false,
    tooling_number TEXT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================================================
-- Enrich machines table with new columns (all nullable)
-- ============================================================

ALTER TABLE machines ADD COLUMN model TEXT;
ALTER TABLE machines ADD COLUMN serial_number TEXT;
ALTER TABLE machines ADD COLUMN machine_type_id UUID REFERENCES machine_types(id) ON DELETE SET NULL;
ALTER TABLE machines ADD COLUMN manufacturer_id UUID REFERENCES manufacturers(id) ON DELETE SET NULL;
ALTER TABLE machines ADD COLUMN location_id UUID REFERENCES locations(id) ON DELETE SET NULL;
ALTER TABLE machines ADD COLUMN project_id UUID REFERENCES projects(id) ON DELETE SET NULL;
ALTER TABLE machines ADD COLUMN responsible TEXT;

-- ============================================================
-- Indexes
-- ============================================================

CREATE INDEX idx_machines_machine_type_id ON machines(machine_type_id);
CREATE INDEX idx_machines_manufacturer_id ON machines(manufacturer_id);
CREATE INDEX idx_machines_location_id ON machines(location_id);
CREATE INDEX idx_machines_project_id ON machines(project_id);
CREATE INDEX idx_purchase_rfqs_machine_id ON purchase_rfqs(machine_id);

-- ============================================================
-- Triggers: auto-update updated_at
-- ============================================================

CREATE TRIGGER trg_machine_types_updated_at
    BEFORE UPDATE ON machine_types
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_manufacturers_updated_at
    BEFORE UPDATE ON manufacturers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_locations_updated_at
    BEFORE UPDATE ON locations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_purchase_rfqs_updated_at
    BEFORE UPDATE ON purchase_rfqs
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
