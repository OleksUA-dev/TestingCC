use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

use shared::{
    models::workflow::{
        Workflow, WorkflowNode, WorkflowEdge, WorkflowVariable, WorkflowExecution,
        CreateWorkflowRequest, CreateNodeRequest, CreateEdgeRequest,
        ValidationResult, TriggerType, ExecutionStatus,
    },
    CoreResult, CoreError,
};

use crate::{
    repository::workflow_repository,
    workflow::{
        validator::WorkflowValidator,
        executor::WorkflowExecutor,
    },
};

// ============================================================================
// Workflow Management
// ============================================================================

/// Create a new workflow
pub async fn create_workflow(
    pool: &PgPool,
    request: CreateWorkflowRequest,
) -> CoreResult<Workflow> {
    let workflow = Workflow {
        id: Uuid::new_v4(),
        name: request.name,
        description: request.description,
        entity_id: request.entity_id,
        trigger_type: request.trigger_type,
        is_active: false, // Start as inactive until validated
        version: 1,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: None, // TODO: Get from auth context
    };

    workflow_repository::create_workflow(pool, &workflow).await?;

    Ok(workflow)
}

/// Get workflow by ID
pub async fn get_workflow(pool: &PgPool, id: Uuid) -> CoreResult<Workflow> {
    workflow_repository::get_workflow_by_id(pool, id)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Workflow with id {} not found", id)))
}

/// List all workflows
pub async fn list_workflows(pool: &PgPool) -> CoreResult<Vec<Workflow>> {
    workflow_repository::list_workflows(pool).await
}

/// Update workflow
pub async fn update_workflow(
    pool: &PgPool,
    id: Uuid,
    name: Option<String>,
    description: Option<String>,
    is_active: Option<bool>,
) -> CoreResult<Workflow> {
    let mut workflow = get_workflow(pool, id).await?;

    if let Some(new_name) = name {
        workflow.name = new_name;
    }
    if let Some(new_description) = description {
        workflow.description = Some(new_description);
    }
    if let Some(new_active) = is_active {
        workflow.is_active = new_active;
    }

    workflow.updated_at = Utc::now();

    workflow_repository::update_workflow(pool, &workflow).await?;

    Ok(workflow)
}

/// Delete workflow
pub async fn delete_workflow(pool: &PgPool, id: Uuid) -> CoreResult<()> {
    workflow_repository::delete_workflow(pool, id).await
}

// ============================================================================
// Node Management
// ============================================================================

/// Add node to workflow
pub async fn create_node(
    pool: &PgPool,
    request: CreateNodeRequest,
) -> CoreResult<WorkflowNode> {
    // Verify workflow exists
    get_workflow(pool, request.workflow_id).await?;

    let node = WorkflowNode {
        id: Uuid::new_v4(),
        workflow_id: request.workflow_id,
        node_type: request.node_type,
        name: request.name,
        config: request.config,
        position_x: request.position_x,
        position_y: request.position_y,
        created_at: Utc::now(),
    };

    workflow_repository::create_node(pool, &node).await?;

    Ok(node)
}

/// Get all nodes for a workflow
pub async fn get_workflow_nodes(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<Vec<WorkflowNode>> {
    workflow_repository::get_nodes_by_workflow(pool, workflow_id).await
}

/// Update node
pub async fn update_node(
    pool: &PgPool,
    node_id: Uuid,
    name: Option<String>,
    config: Option<serde_json::Value>,
    position_x: Option<f64>,
    position_y: Option<f64>,
) -> CoreResult<WorkflowNode> {
    // Get existing node
    let nodes = workflow_repository::get_nodes_by_workflow(
        pool,
        Uuid::nil(), // We need to get workflow_id first
    ).await?;

    let mut node = nodes
        .into_iter()
        .find(|n| n.id == node_id)
        .ok_or_else(|| CoreError::NotFound(format!("Node with id {} not found", node_id)))?;

    if let Some(new_name) = name {
        node.name = new_name;
    }
    if let Some(new_config) = config {
        node.config = new_config;
    }
    if let Some(new_x) = position_x {
        node.position_x = new_x;
    }
    if let Some(new_y) = position_y {
        node.position_y = new_y;
    }

    workflow_repository::update_node(pool, &node).await?;

    Ok(node)
}

/// Delete node
pub async fn delete_node(pool: &PgPool, node_id: Uuid) -> CoreResult<()> {
    workflow_repository::delete_node(pool, node_id).await
}

// ============================================================================
// Edge Management
// ============================================================================

/// Create edge between nodes
pub async fn create_edge(
    pool: &PgPool,
    request: CreateEdgeRequest,
) -> CoreResult<WorkflowEdge> {
    // Verify workflow exists
    get_workflow(pool, request.workflow_id).await?;

    let edge = WorkflowEdge {
        id: Uuid::new_v4(),
        workflow_id: request.workflow_id,
        source_node_id: request.source_node_id,
        target_node_id: request.target_node_id,
        condition: request.condition,
        label: request.label,
        created_at: Utc::now(),
    };

    workflow_repository::create_edge(pool, &edge).await?;

    Ok(edge)
}

/// Get all edges for a workflow
pub async fn get_workflow_edges(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<Vec<WorkflowEdge>> {
    workflow_repository::get_edges_by_workflow(pool, workflow_id).await
}

/// Delete edge
pub async fn delete_edge(pool: &PgPool, edge_id: Uuid) -> CoreResult<()> {
    workflow_repository::delete_edge(pool, edge_id).await
}

// ============================================================================
// Variable Management
// ============================================================================

/// Get all variables for a workflow
pub async fn get_workflow_variables(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<Vec<WorkflowVariable>> {
    workflow_repository::get_variables_by_workflow(pool, workflow_id).await
}

// ============================================================================
// Workflow Validation
// ============================================================================

/// Validate workflow configuration
pub async fn validate_workflow(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<ValidationResult> {
    let workflow = get_workflow(pool, workflow_id).await?;
    let nodes = get_workflow_nodes(pool, workflow_id).await?;
    let edges = get_workflow_edges(pool, workflow_id).await?;
    let variables = get_workflow_variables(pool, workflow_id).await?;

    let validator = WorkflowValidator::new(workflow, nodes, edges, variables);
    Ok(validator.validate())
}

// ============================================================================
// Workflow Execution
// ============================================================================

/// Execute a workflow
pub async fn execute_workflow(
    pool: &PgPool,
    workflow_id: Uuid,
    initial_context: Option<HashMap<String, serde_json::Value>>,
) -> CoreResult<WorkflowExecution> {
    // Get workflow and verify it's active
    let workflow = get_workflow(pool, workflow_id).await?;

    if !workflow.is_active {
        return Err(CoreError::InvalidInput(
            "Cannot execute inactive workflow".to_string()
        ));
    }

    // Validate workflow before execution
    let validation = validate_workflow(pool, workflow_id).await?;
    if !validation.is_valid {
        return Err(CoreError::InvalidInput(format!(
            "Workflow validation failed: {:?}",
            validation.errors
        )));
    }

    // Get workflow components
    let nodes = get_workflow_nodes(pool, workflow_id).await?;
    let edges = get_workflow_edges(pool, workflow_id).await?;
    let variables = get_workflow_variables(pool, workflow_id).await?;

    // Create execution record
    let execution = WorkflowExecution {
        id: Uuid::new_v4(),
        workflow_id,
        status: ExecutionStatus::Pending,
        context: serde_json::json!({}),
        current_node_id: None,
        started_at: Utc::now(),
        completed_at: None,
        error: None,
    };

    workflow_repository::create_execution(pool, &execution).await?;

    // Execute workflow asynchronously
    // In production, this would be handled by a job queue
    let executor = WorkflowExecutor::new(workflow, nodes, edges, variables);

    match executor.execute(initial_context).await {
        Ok(context) => {
            // Update execution with results
            let updated_execution = WorkflowExecution {
                id: execution.id,
                workflow_id,
                status: context.status,
                context: serde_json::to_value(&context.variables)?,
                current_node_id: context.current_node_id,
                started_at: execution.started_at,
                completed_at: Some(Utc::now()),
                error: context.error,
            };

            workflow_repository::update_execution(pool, &updated_execution).await?;

            Ok(updated_execution)
        }
        Err(err) => {
            // Update execution with error
            let failed_execution = WorkflowExecution {
                id: execution.id,
                workflow_id,
                status: ExecutionStatus::Failed,
                context: execution.context,
                current_node_id: None,
                started_at: execution.started_at,
                completed_at: Some(Utc::now()),
                error: Some(err.to_string()),
            };

            workflow_repository::update_execution(pool, &failed_execution).await?;

            Err(CoreError::Internal(format!("Workflow execution failed: {}", err)))
        }
    }
}

/// Get execution by ID
pub async fn get_execution(pool: &PgPool, execution_id: Uuid) -> CoreResult<WorkflowExecution> {
    workflow_repository::get_execution_by_id(pool, execution_id)
        .await?
        .ok_or_else(|| CoreError::NotFound(format!("Execution with id {} not found", execution_id)))
}

/// Get all executions for a workflow
pub async fn get_workflow_executions(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<Vec<WorkflowExecution>> {
    workflow_repository::get_executions_by_workflow(pool, workflow_id).await
}

/// Get workflow definition with all components
pub async fn get_workflow_full(
    pool: &PgPool,
    workflow_id: Uuid,
) -> CoreResult<WorkflowFullDefinition> {
    let workflow = get_workflow(pool, workflow_id).await?;
    let nodes = get_workflow_nodes(pool, workflow_id).await?;
    let edges = get_workflow_edges(pool, workflow_id).await?;
    let variables = get_workflow_variables(pool, workflow_id).await?;

    Ok(WorkflowFullDefinition {
        workflow,
        nodes,
        edges,
        variables,
    })
}

/// Full workflow definition with all components
#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkflowFullDefinition {
    pub workflow: Workflow,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
    pub variables: Vec<WorkflowVariable>,
}
