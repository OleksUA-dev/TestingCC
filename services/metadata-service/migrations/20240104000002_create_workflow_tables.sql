-- Create workflow trigger type enum
CREATE TYPE trigger_type AS ENUM (
    'manual',
    'on_create',
    'on_update',
    'on_delete',
    'scheduled'
);

-- Create node type enum
CREATE TYPE node_type AS ENUM (
    'start',
    'end',
    'task',
    'decision',
    'fork',
    'join',
    'assignment',
    'loop',
    'api_call',
    'notification',
    'wait'
);

-- Create variable type enum
CREATE TYPE variable_type AS ENUM (
    'string',
    'integer',
    'float',
    'boolean',
    'date',
    'date_time',
    'array',
    'object',
    'any'
);

-- Create execution status enum
CREATE TYPE execution_status AS ENUM (
    'pending',
    'running',
    'completed',
    'failed',
    'cancelled'
);

-- Create workflows table
CREATE TABLE workflows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    entity_id UUID REFERENCES entities(id) ON DELETE SET NULL,
    trigger_type trigger_type NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID
);

-- Create workflow_nodes table
CREATE TABLE workflow_nodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    node_type node_type NOT NULL,
    name VARCHAR(100) NOT NULL,
    position_x DOUBLE PRECISION NOT NULL DEFAULT 0,
    position_y DOUBLE PRECISION NOT NULL DEFAULT 0,
    config JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create workflow_edges table
CREATE TABLE workflow_edges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    source_node_id UUID NOT NULL REFERENCES workflow_nodes(id) ON DELETE CASCADE,
    target_node_id UUID NOT NULL REFERENCES workflow_nodes(id) ON DELETE CASCADE,
    condition TEXT,
    label VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT check_no_self_loop CHECK (source_node_id != target_node_id)
);

-- Create workflow_variables table
CREATE TABLE workflow_variables (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    variable_type variable_type NOT NULL,
    default_value JSONB,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(workflow_id, name)
);

-- Create workflow_executions table
CREATE TABLE workflow_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workflow_id UUID NOT NULL REFERENCES workflows(id) ON DELETE CASCADE,
    status execution_status NOT NULL DEFAULT 'pending',
    context JSONB NOT NULL DEFAULT '{}',
    current_node_id UUID REFERENCES workflow_nodes(id),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    error TEXT
);

-- Create indexes
CREATE INDEX idx_workflows_entity ON workflows(entity_id);
CREATE INDEX idx_workflows_trigger ON workflows(trigger_type);
CREATE INDEX idx_workflows_active ON workflows(is_active);

CREATE INDEX idx_workflow_nodes_workflow ON workflow_nodes(workflow_id);
CREATE INDEX idx_workflow_nodes_type ON workflow_nodes(node_type);

CREATE INDEX idx_workflow_edges_workflow ON workflow_edges(workflow_id);
CREATE INDEX idx_workflow_edges_source ON workflow_edges(source_node_id);
CREATE INDEX idx_workflow_edges_target ON workflow_edges(target_node_id);

CREATE INDEX idx_workflow_variables_workflow ON workflow_variables(workflow_id);

CREATE INDEX idx_workflow_executions_workflow ON workflow_executions(workflow_id);
CREATE INDEX idx_workflow_executions_status ON workflow_executions(status);

-- Add comments
COMMENT ON TABLE workflows IS 'Workflow definitions for business process automation';
COMMENT ON TABLE workflow_nodes IS 'Nodes/steps in workflow (logical blocks)';
COMMENT ON TABLE workflow_edges IS 'Connections between workflow nodes';
COMMENT ON TABLE workflow_variables IS 'Variables used in workflow for type checking';
COMMENT ON TABLE workflow_executions IS 'Workflow execution instances and their state';

COMMENT ON COLUMN workflows.trigger_type IS 'How workflow is triggered (manual, on entity event, scheduled)';
COMMENT ON COLUMN workflow_nodes.config IS 'Node-specific configuration (JSON)';
COMMENT ON COLUMN workflow_edges.condition IS 'Condition for edge traversal (for Decision nodes)';
COMMENT ON COLUMN workflow_executions.context IS 'Execution context with variable values';
