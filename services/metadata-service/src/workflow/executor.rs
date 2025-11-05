use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;
use serde_json::Value;

use shared::models::workflow::{
    ExecutionStatus, NodeType, Workflow, WorkflowEdge, WorkflowExecution, WorkflowNode,
    WorkflowVariable,
};
use shared::CoreResult;

/// Execution context for a workflow run
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub execution_id: Uuid,
    pub workflow_id: Uuid,
    pub variables: HashMap<String, Value>,
    pub current_node_id: Option<Uuid>,
    pub status: ExecutionStatus,
    pub error: Option<String>,
}

impl ExecutionContext {
    pub fn new(execution_id: Uuid, workflow_id: Uuid) -> Self {
        Self {
            execution_id,
            workflow_id,
            variables: HashMap::new(),
            current_node_id: None,
            status: ExecutionStatus::Pending,
            error: None,
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn mark_running(&mut self, node_id: Uuid) {
        self.current_node_id = Some(node_id);
        self.status = ExecutionStatus::Running;
    }

    pub fn mark_completed(&mut self) {
        self.status = ExecutionStatus::Completed;
        self.current_node_id = None;
    }

    pub fn mark_failed(&mut self, error: String) {
        self.status = ExecutionStatus::Failed;
        self.error = Some(error);
    }
}

/// Workflow executor engine
pub struct WorkflowExecutor {
    workflow: Workflow,
    nodes: HashMap<Uuid, WorkflowNode>,
    edges: Vec<WorkflowEdge>,
    variables: Vec<WorkflowVariable>,
}

impl WorkflowExecutor {
    pub fn new(
        workflow: Workflow,
        nodes: Vec<WorkflowNode>,
        edges: Vec<WorkflowEdge>,
        variables: Vec<WorkflowVariable>,
    ) -> Self {
        let nodes_map = nodes.into_iter().map(|n| (n.id, n)).collect();

        Self {
            workflow,
            nodes: nodes_map,
            edges,
            variables,
        }
    }

    /// Start workflow execution
    pub async fn execute(&self, initial_context: Option<HashMap<String, Value>>) -> CoreResult<ExecutionContext> {
        let execution_id = Uuid::new_v4();
        let mut context = ExecutionContext::new(execution_id, self.workflow.id);

        // Initialize variables with default values
        for var in &self.variables {
            if let Some(default_value) = &var.default_value {
                context.set_variable(var.name.clone(), default_value.clone());
            }
        }

        // Override with initial context if provided
        if let Some(initial) = initial_context {
            for (key, value) in initial {
                context.set_variable(key, value);
            }
        }

        // Find start node
        let start_node = self
            .nodes
            .values()
            .find(|n| n.node_type == NodeType::Start)
            .ok_or_else(|| anyhow::anyhow!("No Start node found in workflow"))?;

        // Execute from start node
        self.execute_from_node(start_node.id, &mut context).await?;

        Ok(context)
    }

    /// Execute workflow starting from a specific node
    async fn execute_from_node(
        &self,
        node_id: Uuid,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        let node = self
            .nodes
            .get(&node_id)
            .ok_or_else(|| anyhow::anyhow!("Node {} not found", node_id))?;

        context.mark_running(node_id);

        // Execute node based on type
        match node.node_type {
            NodeType::Start => {
                // Start node just passes through
                self.execute_next_nodes(node_id, context, None).await?;
            }
            NodeType::End => {
                // End node marks execution as completed
                context.mark_completed();
            }
            NodeType::Task => {
                self.execute_task_node(node, context).await?;
                self.execute_next_nodes(node_id, context, None).await?;
            }
            NodeType::Assignment => {
                self.execute_assignment_node(node, context).await?;
                self.execute_next_nodes(node_id, context, None).await?;
            }
            NodeType::Decision => {
                self.execute_decision_node(node, context).await?;
            }
            NodeType::Fork => {
                self.execute_fork_node(node, context).await?;
            }
            NodeType::Join => {
                // Join node waits for all incoming paths (simplified - just pass through)
                self.execute_next_nodes(node_id, context, None).await?;
            }
            NodeType::Loop => {
                self.execute_loop_node(node, context).await?;
            }
            NodeType::ApiCall => {
                self.execute_api_call_node(node, context).await?;
                self.execute_next_nodes(node_id, context, None).await?;
            }
            NodeType::Notification => {
                self.execute_notification_node(node, context).await?;
                self.execute_next_nodes(node_id, context, None).await?;
            }
            NodeType::Wait => {
                self.execute_wait_node(node, context).await?;
                self.execute_next_nodes(node_id, context, None).await?;
            }
        }

        Ok(())
    }

    /// Execute next nodes based on outgoing edges
    async fn execute_next_nodes(
        &self,
        current_node_id: Uuid,
        context: &mut ExecutionContext,
        condition_result: Option<bool>,
    ) -> CoreResult<()> {
        let outgoing_edges: Vec<_> = self
            .edges
            .iter()
            .filter(|e| e.source_node_id == current_node_id)
            .collect();

        for edge in outgoing_edges {
            // Check edge condition if present
            if let Some(condition) = &edge.condition {
                if let Some(result) = condition_result {
                    // For decision nodes, execute appropriate branch
                    let should_execute = if condition.contains("true") || condition.contains("yes") {
                        result
                    } else {
                        !result
                    };

                    if !should_execute {
                        continue;
                    }
                } else {
                    // Evaluate condition
                    if !self.evaluate_condition(condition, context)? {
                        continue;
                    }
                }
            }

            self.execute_from_node(edge.target_node_id, context).await?;
        }

        Ok(())
    }

    /// Execute a task node
    async fn execute_task_node(
        &self,
        node: &WorkflowNode,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        // Extract task details from config
        let task_name = node.config.get("task_name")
            .and_then(|v| v.as_str())
            .unwrap_or(&node.name);

        tracing::info!("Executing task: {}", task_name);

        // In a real implementation, this would delegate to a task service
        // For now, just log and continue
        Ok(())
    }

    /// Execute an assignment node
    async fn execute_assignment_node(
        &self,
        node: &WorkflowNode,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        let variable = node
            .config
            .get("variable")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Assignment node missing 'variable' config"))?;

        let value = node
            .config
            .get("value")
            .ok_or_else(|| anyhow::anyhow!("Assignment node missing 'value' config"))?;

        // Resolve value if it's a variable reference
        let resolved_value = self.resolve_value(value, context)?;

        context.set_variable(variable.to_string(), resolved_value);

        tracing::info!("Assigned {} = {:?}", variable, value);
        Ok(())
    }

    /// Execute a decision node
    async fn execute_decision_node(
        &self,
        node: &WorkflowNode,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        let condition = node
            .config
            .get("condition")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Decision node missing 'condition' config"))?;

        let result = self.evaluate_condition(condition, context)?;

        tracing::info!("Decision node '{}' evaluated to: {}", node.name, result);

        // Execute next nodes based on condition result
        self.execute_next_nodes(node.id, context, Some(result)).await?;

        Ok(())
    }

    /// Execute a fork node (parallel execution)
    async fn execute_fork_node(
        &self,
        node: &WorkflowNode,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        // In a real implementation, this would spawn parallel tasks
        // For now, execute sequentially
        tracing::info!("Fork node '{}' - executing parallel branches", node.name);

        self.execute_next_nodes(node.id, context, None).await?;

        Ok(())
    }

    /// Execute a loop node
    async fn execute_loop_node(
        &self,
        node: &WorkflowNode,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        let max_iterations = node
            .config
            .get("max_iterations")
            .and_then(|v| v.as_i64())
            .unwrap_or(100) as usize;

        let condition = node.config.get("condition").and_then(|v| v.as_str());

        let mut iteration = 0;

        loop {
            // Check max iterations
            if iteration >= max_iterations {
                tracing::warn!("Loop '{}' reached max iterations: {}", node.name, max_iterations);
                break;
            }

            // Check condition if present
            if let Some(cond) = condition {
                if !self.evaluate_condition(cond, context)? {
                    break;
                }
            } else {
                // No condition means run max_iterations times
                if iteration >= max_iterations {
                    break;
                }
            }

            // Execute loop body
            self.execute_next_nodes(node.id, context, None).await?;

            iteration += 1;

            // Safety check: prevent infinite loops beyond max_iterations
            if iteration > 10000 {
                return Err(anyhow::anyhow!(
                    "Loop '{}' exceeded safety limit of 10000 iterations",
                    node.name
                ).into());
            }
        }

        tracing::info!("Loop '{}' completed after {} iterations", node.name, iteration);
        Ok(())
    }

    /// Execute an API call node
    async fn execute_api_call_node(
        &self,
        node: &WorkflowNode,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        let url = node
            .config
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("ApiCall node missing 'url' config"))?;

        let method = node
            .config
            .get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("GET");

        tracing::info!("API call: {} {}", method, url);

        // In a real implementation, this would make an actual HTTP request
        // For now, just simulate success
        let response = serde_json::json!({
            "status": "success",
            "data": {}
        });

        // Store response in context if response_variable is specified
        if let Some(response_var) = node.config.get("response_variable").and_then(|v| v.as_str()) {
            context.set_variable(response_var.to_string(), response);
        }

        Ok(())
    }

    /// Execute a notification node
    async fn execute_notification_node(
        &self,
        node: &WorkflowNode,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        let message = node
            .config
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Notification");

        tracing::info!("Notification: {}", message);

        // In a real implementation, this would send actual notifications
        Ok(())
    }

    /// Execute a wait node
    async fn execute_wait_node(
        &self,
        node: &WorkflowNode,
        context: &mut ExecutionContext,
    ) -> CoreResult<()> {
        let duration = node
            .config
            .get("duration_seconds")
            .and_then(|v| v.as_i64())
            .unwrap_or(1);

        tracing::info!("Wait node: sleeping for {} seconds", duration);

        // In a real implementation, this would schedule continuation after wait
        // For now, just continue immediately
        Ok(())
    }

    /// Evaluate a condition expression
    fn evaluate_condition(
        &self,
        condition: &str,
        context: &ExecutionContext,
    ) -> CoreResult<bool> {
        // Simplified condition evaluation
        // In a real implementation, use a proper expression parser

        // Replace variable references with actual values
        let mut resolved = condition.to_string();

        let var_pattern = regex::Regex::new(r"\$\{(\w+)\}").unwrap();
        for cap in var_pattern.captures_iter(condition) {
            let var_name = &cap[1];
            if let Some(value) = context.get_variable(var_name) {
                let value_str = match value {
                    Value::String(s) => format!("\"{}\"", s),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => value.to_string(),
                };
                resolved = resolved.replace(&cap[0], &value_str);
            }
        }

        // Simple evaluation for common patterns
        if resolved.contains("==") {
            let parts: Vec<&str> = resolved.split("==").collect();
            if parts.len() == 2 {
                return Ok(parts[0].trim() == parts[1].trim());
            }
        }

        if resolved.contains(">") {
            let parts: Vec<&str> = resolved.split(">").collect();
            if parts.len() == 2 {
                if let (Ok(left), Ok(right)) = (
                    parts[0].trim().parse::<f64>(),
                    parts[1].trim().parse::<f64>(),
                ) {
                    return Ok(left > right);
                }
            }
        }

        if resolved.contains("<") {
            let parts: Vec<&str> = resolved.split("<").collect();
            if parts.len() == 2 {
                if let (Ok(left), Ok(right)) = (
                    parts[0].trim().parse::<f64>(),
                    parts[1].trim().parse::<f64>(),
                ) {
                    return Ok(left < right);
                }
            }
        }

        // Default: try to parse as boolean
        match resolved.trim().to_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Ok(false),
        }
    }

    /// Resolve a value (may be a literal or variable reference)
    fn resolve_value(&self, value: &Value, context: &ExecutionContext) -> CoreResult<Value> {
        match value {
            Value::String(s) => {
                // Check if it's a variable reference
                if s.starts_with("${") && s.ends_with("}") {
                    let var_name = &s[2..s.len() - 1];
                    context
                        .get_variable(var_name)
                        .cloned()
                        .ok_or_else(|| anyhow::anyhow!("Variable '{}' not found", var_name).into())
                } else {
                    Ok(value.clone())
                }
            }
            _ => Ok(value.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::models::workflow::TriggerType;

    fn create_test_workflow() -> Workflow {
        Workflow {
            id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            description: None,
            entity_id: None,
            trigger_type: TriggerType::Manual,
            is_active: true,
            version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_simple_execution() {
        let workflow = create_test_workflow();

        let start_node = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            node_type: NodeType::Start,
            name: "Start".to_string(),
            config: serde_json::json!({}),
            position_x: 0.0,
            position_y: 0.0,
            created_at: Utc::now(),
        };

        let end_node = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            node_type: NodeType::End,
            name: "End".to_string(),
            config: serde_json::json!({}),
            position_x: 100.0,
            position_y: 0.0,
            created_at: Utc::now(),
        };

        let edge = WorkflowEdge {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            source_node_id: start_node.id,
            target_node_id: end_node.id,
            condition: None,
            order: 0,
            created_at: Utc::now(),
        };

        let executor = WorkflowExecutor::new(
            workflow,
            vec![start_node, end_node],
            vec![edge],
            vec![],
        );

        let result = executor.execute(None).await;
        assert!(result.is_ok());

        let context = result.unwrap();
        assert_eq!(context.status, ExecutionStatus::Completed);
    }

    #[tokio::test]
    async fn test_assignment_execution() {
        let workflow = create_test_workflow();

        let start_node = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            node_type: NodeType::Start,
            name: "Start".to_string(),
            config: serde_json::json!({}),
            position_x: 0.0,
            position_y: 0.0,
            created_at: Utc::now(),
        };

        let assign_node = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            node_type: NodeType::Assignment,
            name: "Set Name".to_string(),
            config: serde_json::json!({
                "variable": "name",
                "value": "John Doe"
            }),
            position_x: 50.0,
            position_y: 0.0,
            created_at: Utc::now(),
        };

        let end_node = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            node_type: NodeType::End,
            name: "End".to_string(),
            config: serde_json::json!({}),
            position_x: 100.0,
            position_y: 0.0,
            created_at: Utc::now(),
        };

        let edges = vec![
            WorkflowEdge {
                id: Uuid::new_v4(),
                workflow_id: workflow.id,
                source_node_id: start_node.id,
                target_node_id: assign_node.id,
                condition: None,
                order: 0,
                created_at: Utc::now(),
            },
            WorkflowEdge {
                id: Uuid::new_v4(),
                workflow_id: workflow.id,
                source_node_id: assign_node.id,
                target_node_id: end_node.id,
                condition: None,
                order: 0,
                created_at: Utc::now(),
            },
        ];

        let executor = WorkflowExecutor::new(
            workflow,
            vec![start_node, assign_node, end_node],
            edges,
            vec![],
        );

        let result = executor.execute(None).await;
        assert!(result.is_ok());

        let context = result.unwrap();
        assert_eq!(context.status, ExecutionStatus::Completed);
        assert_eq!(
            context.get_variable("name"),
            Some(&Value::String("John Doe".to_string()))
        );
    }

    #[test]
    fn test_condition_evaluation() {
        let workflow = create_test_workflow();
        let executor = WorkflowExecutor::new(workflow, vec![], vec![], vec![]);

        let mut context = ExecutionContext::new(Uuid::new_v4(), Uuid::new_v4());
        context.set_variable("age".to_string(), Value::Number(25.into()));

        let result = executor.evaluate_condition("${age} > 18", &context);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
