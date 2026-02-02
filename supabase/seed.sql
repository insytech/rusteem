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

-- Seed: machine types
INSERT INTO machine_types (name, description) VALUES
    ('Press', 'Hydraulic and mechanical presses'),
    ('Robot', 'Industrial robotic arms and systems'),
    ('Conveyor', 'Belt and roller conveyor systems'),
    ('CNC', 'Computer numerical control machines'),
    ('Welding Station', 'Automated and manual welding equipment'),
    ('Assembly Cell', 'Semi-automated assembly workstations'),
    ('Inspection', 'Quality inspection and measurement equipment'),
    ('Packaging', 'End-of-line packaging machines')
ON CONFLICT (name) DO NOTHING;

-- Seed: manufacturers
INSERT INTO manufacturers (name, website) VALUES
    ('FANUC', 'https://www.fanuc.com'),
    ('ABB', 'https://www.abb.com'),
    ('KUKA', 'https://www.kuka.com'),
    ('Siemens', 'https://www.siemens.com'),
    ('Bosch Rexroth', 'https://www.boschrexroth.com'),
    ('Schuler', 'https://www.schulergroup.com')
ON CONFLICT (name) DO NOTHING;

-- Seed: locations
INSERT INTO locations (area, line) VALUES
    ('Body Shop', 'Line 1'),
    ('Body Shop', 'Line 2'),
    ('Paint Shop', 'Line 1'),
    ('Assembly', 'Line 1'),
    ('Assembly', 'Line 2'),
    ('Quality', 'Inspection Bay')
ON CONFLICT (area, line) DO NOTHING;

-- Seed: projects
INSERT INTO projects (name, code, description) VALUES
    ('2026 Model Refresh', 'MR-2026', 'Annual model year tooling and equipment updates'),
    ('New Paint Line', 'NPL-01', 'Installation of new paint shop line 2'),
    ('Automation Phase 3', 'AP3', 'Third phase of assembly automation rollout')
ON CONFLICT (code) DO NOTHING;

-- Seed: team members
INSERT INTO team_members (name, email, role, department) VALUES
    ('Carlos Mendez', 'carlos.mendez@example.com', 'Engineer', 'Manufacturing'),
    ('Anna Schmidt', 'anna.schmidt@example.com', 'NPI Lead', 'New Programs'),
    ('James Park', 'james.park@example.com', 'Maintenance', 'Facilities'),
    ('Sofia Reyes', 'sofia.reyes@example.com', 'Quality Auditor', 'Quality'),
    ('Liam Chen', 'liam.chen@example.com', 'Engineering Lead', 'Engineering')
ON CONFLICT (email) DO NOTHING;

-- Seed: create storage bucket for documents
INSERT INTO storage.buckets (id, name, public)
VALUES ('documents', 'documents', false)
ON CONFLICT (id) DO NOTHING;
