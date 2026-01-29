-- Add workflow_id to approvals table for tracking which workflow an approval belongs to
ALTER TABLE approvals
    ADD COLUMN workflow_id UUID REFERENCES approval_workflows(id) ON DELETE CASCADE;

-- Index for querying approvals by workflow
CREATE INDEX IF NOT EXISTS idx_approvals_workflow_id ON approvals(workflow_id);

-- Index for querying pending approvals efficiently
CREATE INDEX IF NOT EXISTS idx_approvals_decision ON approvals(decision);
