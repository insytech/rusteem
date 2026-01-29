-- Seed: document types
INSERT INTO document_types (name, description, required_for_release) VALUES
    ('3D Drawings', 'CAD 3D models and assemblies', true),
    ('BluePrints', '2D technical drawings with dimensions', true),
    ('Electrical Schematics', 'Wiring diagrams and electrical layouts', true),
    ('Operation Manual', 'Standard operating procedures', true),
    ('Maintenance Manual', 'Preventive and corrective maintenance guides', false),
    ('Spare Parts List', 'Bill of materials and spare parts catalog', false),
    ('Risk Assessment', 'Safety and risk analysis documents', true),
    ('Validation Protocol', 'IQ/OQ/PQ validation documents', true)
ON CONFLICT (name) DO NOTHING;

-- Seed: default approval workflow
INSERT INTO approval_workflows (id, name, description) VALUES
    ('00000000-0000-0000-0000-000000000001', 'Standard Document Approval', 'Default workflow for technical document review and approval');

INSERT INTO approval_steps (workflow_id, step_order, role_name, is_required) VALUES
    ('00000000-0000-0000-0000-000000000001', 1, 'Engineering Lead', true),
    ('00000000-0000-0000-0000-000000000001', 2, 'Quality Auditor', true),
    ('00000000-0000-0000-0000-000000000001', 3, 'Maintenance Manager', false);

-- Seed: create storage bucket for documents
INSERT INTO storage.buckets (id, name, public)
VALUES ('documents', 'documents', false)
ON CONFLICT (id) DO NOTHING;
