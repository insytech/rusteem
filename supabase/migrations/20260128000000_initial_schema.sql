-- 1. Tablas Maestras
CREATE TABLE machines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    asset_number TEXT UNIQUE,
    line TEXT,
    station TEXT,
    area TEXT,
    active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

-- 2. Gestión de Documentos
CREATE TABLE document_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE, -- ej: '3D Drawings', 'BluePrints'
    description TEXT,
    required_for_release BOOLEAN DEFAULT true
);

CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    machine_id UUID REFERENCES machines(id),
    document_type_id UUID REFERENCES document_types(id),
    title TEXT NOT NULL,
    storage_path TEXT NOT NULL, -- Ruta en el Bucket de Supabase
    revision INTEGER DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'draft', -- draft, pending, approved, rejected, archived
    uploader_id UUID NOT NULL, -- UUID de auth.users de Supabase
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

-- 3. Motor de Aprobación (Escalable)
CREATE TABLE approval_workflows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT
);

CREATE TABLE approval_steps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID REFERENCES approval_workflows(id),
    step_order INTEGER NOT NULL,
    role_name TEXT NOT NULL, -- ej: 'Maintenance Manager', 'Quality Auditor'
    is_required BOOLEAN DEFAULT true,
    UNIQUE(workflow_id, step_order)
);

CREATE TABLE approvals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID REFERENCES documents(id),
    step_id UUID REFERENCES approval_steps(id),
    approver_id UUID, -- NULL hasta que alguien lo tome o se asigne
    decision TEXT DEFAULT 'pending', -- pending, approved, rejected
    comments TEXT,
    decided_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now()
);

-- 4. Historial de Aprobaciones (Auditoría)
CREATE TABLE approval_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID REFERENCES documents(id),
    user_id UUID NOT NULL,
    action TEXT NOT NULL, -- ej: 'upload', 'approved', 'rejected', 'comment'
    notes TEXT,
    created_at TIMESTAMPTZ DEFAULT now()
);

-- 5. Mantenimiento (Unificado)
CREATE TABLE maintenance_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    machine_id UUID REFERENCES machines(id),
    description TEXT NOT NULL,
    frequency_value INTEGER NOT NULL,
    frequency_unit TEXT NOT NULL, -- 'hours', 'days', 'months', 'cycles'
    last_performed_at TIMESTAMPTZ,
    next_due_at TIMESTAMPTZ,
    is_enabled BOOLEAN DEFAULT true
);
