-- Machine pipeline tracking for dashboard
-- Tracks each machine's progress through the 7 approval stages

CREATE TYPE stage_name AS ENUM (
    'scope_approval', 'po_trail', 'design', 'run_off',
    'support_documents', 'ramp_up', 'release'
);

CREATE TYPE stage_status AS ENUM (
    'not_started', 'in_progress', 'completed', 'breach', 'overdue'
);

CREATE TABLE machine_approval_stages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    machine_id UUID NOT NULL REFERENCES machines(id) ON DELETE CASCADE,
    stage stage_name NOT NULL,
    status stage_status NOT NULL DEFAULT 'not_started',
    due_date TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(machine_id, stage)
);

CREATE TABLE machine_pipeline_status (
    machine_id UUID PRIMARY KEY REFERENCES machines(id) ON DELETE CASCADE,
    current_stage stage_name NOT NULL DEFAULT 'scope_approval',
    overall_status TEXT NOT NULL DEFAULT 'in_progress',
    release_level INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_machine_approval_stages_machine ON machine_approval_stages(machine_id);
CREATE INDEX idx_machine_approval_stages_status ON machine_approval_stages(status);
CREATE INDEX idx_machine_pipeline_status_stage ON machine_pipeline_status(current_stage);
CREATE INDEX idx_machine_pipeline_status_overall ON machine_pipeline_status(overall_status);
