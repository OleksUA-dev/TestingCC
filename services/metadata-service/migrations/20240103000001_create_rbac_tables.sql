-- Create roles table
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create entity_permissions table
CREATE TABLE entity_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    entity_id UUID NOT NULL REFERENCES entities(id) ON DELETE CASCADE,
    can_create BOOLEAN NOT NULL DEFAULT FALSE,
    can_read BOOLEAN NOT NULL DEFAULT FALSE,
    can_update BOOLEAN NOT NULL DEFAULT FALSE,
    can_delete BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(role_id, entity_id)
);

-- Create user_roles table (cross-reference with api-gateway users)
CREATE TABLE user_roles (
    user_id UUID NOT NULL,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- Create indexes
CREATE INDEX idx_entity_permissions_role ON entity_permissions(role_id);
CREATE INDEX idx_entity_permissions_entity ON entity_permissions(entity_id);
CREATE INDEX idx_user_roles_user ON user_roles(user_id);
CREATE INDEX idx_user_roles_role ON user_roles(role_id);

-- Create default roles
INSERT INTO roles (id, name, description, is_system) VALUES
    (gen_random_uuid(), 'Administrator', 'Full system access', TRUE),
    (gen_random_uuid(), 'User', 'Standard user access', TRUE),
    (gen_random_uuid(), 'Guest', 'Read-only access', TRUE);

-- Add comments
COMMENT ON TABLE roles IS 'User roles for RBAC';
COMMENT ON TABLE entity_permissions IS 'Entity-level permissions for roles';
COMMENT ON TABLE user_roles IS 'User-role assignments';
