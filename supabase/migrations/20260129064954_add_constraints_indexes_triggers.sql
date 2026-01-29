-- ============================================================
-- CHECK constraints
-- ============================================================

ALTER TABLE documents
    ADD CONSTRAINT chk_documents_status
    CHECK (status IN ('draft', 'pending', 'approved', 'rejected', 'archived'));

ALTER TABLE approvals
    ADD CONSTRAINT chk_approvals_decision
    CHECK (decision IN ('pending', 'approved', 'rejected'));

ALTER TABLE maintenance_plans
    ADD CONSTRAINT chk_maintenance_frequency_unit
    CHECK (frequency_unit IN ('hours', 'days', 'months', 'cycles'));

ALTER TABLE maintenance_plans
    ADD CONSTRAINT chk_maintenance_frequency_value_positive
    CHECK (frequency_value > 0);

-- ============================================================
-- ON DELETE policies
-- ============================================================

ALTER TABLE documents
    DROP CONSTRAINT IF EXISTS documents_machine_id_fkey,
    ADD CONSTRAINT documents_machine_id_fkey
    FOREIGN KEY (machine_id) REFERENCES machines(id) ON DELETE SET NULL;

ALTER TABLE documents
    DROP CONSTRAINT IF EXISTS documents_document_type_id_fkey,
    ADD CONSTRAINT documents_document_type_id_fkey
    FOREIGN KEY (document_type_id) REFERENCES document_types(id) ON DELETE RESTRICT;

ALTER TABLE approvals
    DROP CONSTRAINT IF EXISTS approvals_document_id_fkey,
    ADD CONSTRAINT approvals_document_id_fkey
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE;

ALTER TABLE approvals
    DROP CONSTRAINT IF EXISTS approvals_step_id_fkey,
    ADD CONSTRAINT approvals_step_id_fkey
    FOREIGN KEY (step_id) REFERENCES approval_steps(id) ON DELETE CASCADE;

ALTER TABLE approval_steps
    DROP CONSTRAINT IF EXISTS approval_steps_workflow_id_fkey,
    ADD CONSTRAINT approval_steps_workflow_id_fkey
    FOREIGN KEY (workflow_id) REFERENCES approval_workflows(id) ON DELETE CASCADE;

ALTER TABLE approval_history
    DROP CONSTRAINT IF EXISTS approval_history_document_id_fkey,
    ADD CONSTRAINT approval_history_document_id_fkey
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE;

ALTER TABLE maintenance_plans
    DROP CONSTRAINT IF EXISTS maintenance_plans_machine_id_fkey,
    ADD CONSTRAINT maintenance_plans_machine_id_fkey
    FOREIGN KEY (machine_id) REFERENCES machines(id) ON DELETE CASCADE;

-- ============================================================
-- Indexes for common queries
-- ============================================================

CREATE INDEX idx_documents_machine_id ON documents(machine_id);
CREATE INDEX idx_documents_status ON documents(status);
CREATE INDEX idx_documents_document_type_id ON documents(document_type_id);
CREATE INDEX idx_approvals_document_id ON approvals(document_id);
CREATE INDEX idx_approval_history_document_id ON approval_history(document_id);
CREATE INDEX idx_maintenance_plans_machine_id ON maintenance_plans(machine_id);
CREATE INDEX idx_maintenance_plans_next_due_at ON maintenance_plans(next_due_at);

-- ============================================================
-- Trigger: auto-update updated_at
-- ============================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_machines_updated_at
    BEFORE UPDATE ON machines
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_documents_updated_at
    BEFORE UPDATE ON documents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
