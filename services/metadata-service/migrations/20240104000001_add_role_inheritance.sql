-- Add parent_role_id column for role inheritance
ALTER TABLE roles ADD COLUMN parent_role_id UUID REFERENCES roles(id) ON DELETE SET NULL;

-- Create index for parent lookups
CREATE INDEX idx_roles_parent ON roles(parent_role_id);

-- Add comment
COMMENT ON COLUMN roles.parent_role_id IS 'Parent role for inheritance - inherits all permissions from parent';
