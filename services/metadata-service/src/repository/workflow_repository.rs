use sqlx::PgPool;
use uuid::Uuid;

use shared::{
    models::workflow::{
        ExecutionStatus, NodeType, TriggerType, VariableType, Workflow, WorkflowEdge,
        WorkflowExecution, WorkflowNode, WorkflowVariable,
    },
    CoreResult,
};

// ============================================================================
// Workflow CRUD Operations
// ============================================================================

/// Create a new workflow
pub async fn create_workflow(pool: &PgPool, workflow: &Workflow) -> CoreResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO workflows (
            id, name, description, entity_id, trigger_type, is_active, version, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        workflow.id,
        workflow.name,
        workflow.description,
        workflow.entity_id,
        workflow.trigger_type.to_string(),
        workflow.is_active,
        workflow.version,
        workflow.created_at,
        workflow.updated_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get workflow by ID
pub async fn get_workflow_by_id(pool: &PgPool, id: Uuid) -> CoreResult<Option<Workflow>> {
    let row = sqlx::query!(
        r#"
        SELECT id, name, description, entity_id, trigger_type, is_active, version, created_at, updated_at
        FROM workflows
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Workflow {
        id: r.id,
        name: r.name,
        description: r.description,
        entity_id: r.entity_id,
        trigger_type: parse_trigger_type(&r.trigger_type),
        is_active: r.is_active,
        version: r.version,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

/// List all workflows
pub async fn list_workflows(pool: &PgPool) -> CoreResult<Vec<Workflow>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, description, entity_id, trigger_type, is_active, version, created_at, updated_at
        FROM workflows
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| Workflow {
            id: r.id,
            name: r.name,
            description: r.description,
            entity_id: r.entity_id,
            trigger_type: parse_trigger_type(&r.trigger_type),
            is_active: r.is_active,
            version: r.version,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect())
}

/// Update workflow
pub async fn update_workflow(pool: &PgPool, workflow: &Workflow) -> CoreResult<()> {
    sqlx::query!(
        r#"
        UPDATE workflows
        SET name = $2,
            description = $3,
            entity_id = $4,
            trigger_type = $5,
            is_active = $6,
            updated_at = $7
        WHERE id = $1
        "#,
        workflow.id,
        workflow.name,
        workflow.description,
        workflow.entity_id,
        workflow.trigger_type.to_string(),
        workflow.is_active,
        workflow.updated_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete workflow
pub async fn delete_workflow(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM workflows WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// WorkflowNode CRUD Operations
// ============================================================================

/// Create a workflow node
pub async fn create_node(pool: &PgPool, node: &WorkflowNode) -> CoreResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO workflow_nodes (
            id, workflow_id, node_type, name, config, position_x, position_y, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        node.id,
        node.workflow_id,
        node.node_type.to_string(),
        node.name,
        node.config,
        node.position_x,
        node.position_y,
        node.created_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get all nodes for a workflow
pub async fn get_nodes_by_workflow(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<Vec<WorkflowNode>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, workflow_id, node_type, name, config, position_x, position_y, created_at
        FROM workflow_nodes
        WHERE workflow_id = $1
        ORDER BY created_at
        "#,
        workflow_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| WorkflowNode {
            id: r.id,
            workflow_id: r.workflow_id,
            node_type: parse_node_type(&r.node_type),
            name: r.name,
            config: r.config,
            position_x: r.position_x,
            position_y: r.position_y,
            created_at: r.created_at,
        })
        .collect())
}

/// Update workflow node
pub async fn update_node(pool: &PgPool, node: &WorkflowNode) -> CoreResult<()> {
    sqlx::query!(
        r#"
        UPDATE workflow_nodes
        SET name = $2,
            config = $3,
            position_x = $4,
            position_y = $5
        WHERE id = $1
        "#,
        node.id,
        node.name,
        node.config,
        node.position_x,
        node.position_y,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete workflow node
pub async fn delete_node(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM workflow_nodes WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// WorkflowEdge CRUD Operations
// ============================================================================

/// Create a workflow edge
pub async fn create_edge(pool: &PgPool, edge: &WorkflowEdge) -> CoreResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO workflow_edges (
            id, workflow_id, source_node_id, target_node_id, condition, label, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        edge.id,
        edge.workflow_id,
        edge.source_node_id,
        edge.target_node_id,
        edge.condition,
        edge.label,
        edge.created_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get all edges for a workflow
pub async fn get_edges_by_workflow(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<Vec<WorkflowEdge>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, workflow_id, source_node_id, target_node_id, condition, label, created_at
        FROM workflow_edges
        WHERE workflow_id = $1
        ORDER BY label
        "#,
        workflow_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| WorkflowEdge {
            id: r.id,
            workflow_id: r.workflow_id,
            source_node_id: r.source_node_id,
            target_node_id: r.target_node_id,
            condition: r.condition,
            order: r.label,
            created_at: r.created_at,
        })
        .collect())
}

/// Update workflow edge
pub async fn update_edge(pool: &PgPool, edge: &WorkflowEdge) -> CoreResult<()> {
    sqlx::query!(
        r#"
        UPDATE workflow_edges
        SET condition = $2,
            label = $3
        WHERE id = $1
        "#,
        edge.id,
        edge.condition,
        edge.label,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete workflow edge
pub async fn delete_edge(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM workflow_edges WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// WorkflowVariable CRUD Operations
// ============================================================================

/// Create a workflow variable
pub async fn create_variable(pool: &PgPool, variable: &WorkflowVariable) -> CoreResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO workflow_variables (
            id, workflow_id, name, variable_type, default_value, description, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        variable.id,
        variable.workflow_id,
        variable.name,
        variable.variable_type.to_string(),
        variable.default_value,
        variable.description,
        variable.created_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get all variables for a workflow
pub async fn get_variables_by_workflow(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<Vec<WorkflowVariable>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, workflow_id, name, variable_type, default_value, description, created_at
        FROM workflow_variables
        WHERE workflow_id = $1
        ORDER BY created_at
        "#,
        workflow_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| WorkflowVariable {
            id: r.id,
            workflow_id: r.workflow_id,
            name: r.name,
            variable_type: parse_variable_type(&r.variable_type),
            default_value: r.default_value,
            description: r.description,
            created_at: r.created_at,
        })
        .collect())
}

/// Update workflow variable
pub async fn update_variable(pool: &PgPool, variable: &WorkflowVariable) -> CoreResult<()> {
    sqlx::query!(
        r#"
        UPDATE workflow_variables
        SET name = $2,
            variable_type = $3,
            default_value = $4,
            description = $5
        WHERE id = $1
        "#,
        variable.id,
        variable.name,
        variable.variable_type.to_string(),
        variable.default_value,
        variable.description,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete workflow variable
pub async fn delete_variable(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    sqlx::query!(
        r#"
        DELETE FROM workflow_variables WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// WorkflowExecution Operations
// ============================================================================

/// Create a workflow execution
pub async fn create_execution(pool: &PgPool, execution: &WorkflowExecution) -> CoreResult<()> {
    sqlx::query!(
        r#"
        INSERT INTO workflow_executions (
            id, workflow_id, status, context, error, started_at, completed_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        execution.id,
        execution.workflow_id,
        execution.status.to_string(),
        execution.context,
        execution.error,
        execution.started_at,
        execution.completed_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get execution by ID
pub async fn get_execution_by_id(
    pool: &PgPool,
    id: Uuid,
) -> CoreResult<Option<WorkflowExecution>> {
    let row = sqlx::query!(
        r#"
        SELECT id, workflow_id, status, context, error, started_at, completed_at
        FROM workflow_executions
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| WorkflowExecution {
        id: r.id,
        workflow_id: r.workflow_id,
        status: parse_execution_status(&r.status),
        context: r.context,
        error: r.error,
        started_at: r.started_at,
        completed_at: r.completed_at,
    }))
}

/// Get all executions for a workflow
pub async fn get_executions_by_workflow(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<Vec<WorkflowExecution>> {
    let rows = sqlx::query!(
        r#"
        SELECT id, workflow_id, status, context, error, started_at, completed_at
        FROM workflow_executions
        WHERE workflow_id = $1
        ORDER BY started_at DESC
        "#,
        workflow_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| WorkflowExecution {
            id: r.id,
            workflow_id: r.workflow_id,
            status: parse_execution_status(&r.status),
            context: r.context,
            error: r.error,
            started_at: r.started_at,
            completed_at: r.completed_at,
        })
        .collect())
}

/// Update execution status
pub async fn update_execution(pool: &PgPool, execution: &WorkflowExecution) -> CoreResult<()> {
    sqlx::query!(
        r#"
        UPDATE workflow_executions
        SET status = $2,
            context = $3,
            error = $4,
            completed_at = $5
        WHERE id = $1
        "#,
        execution.id,
        execution.status.to_string(),
        execution.context,
        execution.error,
        execution.completed_at,
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================================================
// Helper functions for parsing enum types
// ============================================================================

fn parse_trigger_type(s: &str) -> TriggerType {
    match s.to_lowercase().as_str() {
        "manual" => TriggerType::Manual,
        "oncreate" => TriggerType::OnCreate,
        "onupdate" => TriggerType::OnUpdate,
        "ondelete" => TriggerType::OnDelete,
        "scheduled" => TriggerType::Scheduled,
        _ => TriggerType::Manual,
    }
}

fn parse_node_type(s: &str) -> NodeType {
    match s.to_lowercase().as_str() {
        "start" => NodeType::Start,
        "end" => NodeType::End,
        "task" => NodeType::Task,
        "decision" => NodeType::Decision,
        "fork" => NodeType::Fork,
        "join" => NodeType::Join,
        "assignment" => NodeType::Assignment,
        "loop" => NodeType::Loop,
        "apicall" => NodeType::ApiCall,
        "notification" => NodeType::Notification,
        "wait" => NodeType::Wait,
        _ => NodeType::Task,
    }
}

fn parse_variable_type(s: &str) -> VariableType {
    match s.to_lowercase().as_str() {
        "string" => VariableType::String,
        "integer" => VariableType::Integer,
        "float" => VariableType::Float,
        "boolean" => VariableType::Boolean,
        "date" => VariableType::Date,
        "datetime" => VariableType::DateTime,
        "array" => VariableType::Array,
        "object" => VariableType::Object,
        "any" => VariableType::Any,
        _ => VariableType::Any,
    }
}

fn parse_execution_status(s: &str) -> ExecutionStatus {
    match s.to_lowercase().as_str() {
        "pending" => ExecutionStatus::Pending,
        "running" => ExecutionStatus::Running,
        "completed" => ExecutionStatus::Completed,
        "failed" => ExecutionStatus::Failed,
        "cancelled" => ExecutionStatus::Cancelled,
        _ => ExecutionStatus::Pending,
    }
}
