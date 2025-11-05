use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use shared::models::workflow::{
    NodeType, ValidationError, ValidationErrorType, ValidationResult, Workflow, WorkflowEdge,
    WorkflowNode, WorkflowVariable,
};

use super::cycle_detector::CycleDetector;
use super::type_checker::{TypeChecker, TypeError};

/// Comprehensive workflow validator
pub struct WorkflowValidator {
    workflow: Workflow,
    nodes: Vec<WorkflowNode>,
    edges: Vec<WorkflowEdge>,
    variables: Vec<WorkflowVariable>,
}

impl WorkflowValidator {
    pub fn new(
        workflow: Workflow,
        nodes: Vec<WorkflowNode>,
        edges: Vec<WorkflowEdge>,
        variables: Vec<WorkflowVariable>,
    ) -> Self {
        Self {
            workflow,
            nodes,
            edges,
            variables,
        }
    }

    /// Perform complete validation of the workflow
    pub fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // 1. Structure validation
        errors.extend(self.validate_structure());

        // 2. Connectivity validation
        errors.extend(self.validate_connectivity());

        // 3. Cycle detection (MANDATORY REQUIREMENT)
        if let Some(cycle_error) = self.validate_no_cycles() {
            errors.push(cycle_error);
        }

        // 4. Type checking (MANDATORY REQUIREMENT)
        errors.extend(self.validate_types());

        // 5. Node-specific validation
        errors.extend(self.validate_nodes());

        // 6. Edge-specific validation
        errors.extend(self.validate_edges());

        // 7. Generate warnings
        warnings.extend(self.generate_warnings());

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Validate basic workflow structure
    fn validate_structure(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Must have at least one Start node
        let start_nodes: Vec<_> = self
            .nodes
            .iter()
            .filter(|n| n.node_type == NodeType::Start)
            .collect();

        if start_nodes.is_empty() {
            errors.push(ValidationError {
                error_type: ValidationErrorType::MissingStartNode,
                message: "Workflow must have at least one Start node".to_string(),
                node_id: None,
            });
        }

        if start_nodes.len() > 1 {
            errors.push(ValidationError {
                error_type: ValidationErrorType::MultipleStartNodes,
                message: format!("Workflow has {} Start nodes, expected 1", start_nodes.len()),
                node_id: None,
            });
        }

        // Must have at least one End node
        let end_nodes: Vec<_> = self
            .nodes
            .iter()
            .filter(|n| n.node_type == NodeType::End)
            .collect();

        if end_nodes.is_empty() {
            errors.push(ValidationError {
                error_type: ValidationErrorType::MissingEndNode,
                message: "Workflow must have at least one End node".to_string(),
                node_id: None,
            });
        }

        // Check for empty workflow
        if self.nodes.is_empty() {
            errors.push(ValidationError {
                error_type: ValidationErrorType::InvalidConfiguration,
                message: "Workflow has no nodes".to_string(),
                node_id: None,
            });
        }

        errors
    }

    /// Validate that all nodes are properly connected
    fn validate_connectivity(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Build adjacency lists
        let mut outgoing: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        let mut incoming: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        for edge in &self.edges {
            outgoing
                .entry(edge.source_node_id)
                .or_insert_with(Vec::new)
                .push(edge.target_node_id);
            incoming
                .entry(edge.target_node_id)
                .or_insert_with(Vec::new)
                .push(edge.source_node_id);
        }

        // Check each node
        for node in &self.nodes {
            match node.node_type {
                NodeType::Start => {
                    // Start node should have no incoming edges
                    if incoming.get(&node.id).map(|v| !v.is_empty()).unwrap_or(false) {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: "Start node cannot have incoming edges".to_string(),
                            node_id: Some(node.id),
                        });
                    }

                    // Start node must have at least one outgoing edge
                    if !outgoing.get(&node.id).map(|v| !v.is_empty()).unwrap_or(false) {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::DisconnectedNode,
                            message: "Start node must have at least one outgoing edge".to_string(),
                            node_id: Some(node.id),
                        });
                    }
                }
                NodeType::End => {
                    // End node should have no outgoing edges
                    if outgoing.get(&node.id).map(|v| !v.is_empty()).unwrap_or(false) {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: "End node cannot have outgoing edges".to_string(),
                            node_id: Some(node.id),
                        });
                    }

                    // End node must have at least one incoming edge
                    if !incoming.get(&node.id).map(|v| !v.is_empty()).unwrap_or(false) {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::DisconnectedNode,
                            message: "End node must have at least one incoming edge".to_string(),
                            node_id: Some(node.id),
                        });
                    }
                }
                NodeType::Fork => {
                    // Fork node must have multiple outgoing edges
                    let out_count = outgoing.get(&node.id).map(|v| v.len()).unwrap_or(0);
                    if out_count < 2 {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: format!(
                                "Fork node must have at least 2 outgoing edges, has {}",
                                out_count
                            ),
                            node_id: Some(node.id),
                        });
                    }
                }
                NodeType::Join => {
                    // Join node must have multiple incoming edges
                    let in_count = incoming.get(&node.id).map(|v| v.len()).unwrap_or(0);
                    if in_count < 2 {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: format!(
                                "Join node must have at least 2 incoming edges, has {}",
                                in_count
                            ),
                            node_id: Some(node.id),
                        });
                    }
                }
                NodeType::Decision => {
                    // Decision node should have exactly 2 outgoing edges (true/false branches)
                    let out_count = outgoing.get(&node.id).map(|v| v.len()).unwrap_or(0);
                    if out_count != 2 {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: format!(
                                "Decision node should have exactly 2 outgoing edges, has {}",
                                out_count
                            ),
                            node_id: Some(node.id),
                        });
                    }
                }
                _ => {
                    // Other nodes should have at least one incoming and one outgoing edge
                    if !incoming.get(&node.id).map(|v| !v.is_empty()).unwrap_or(false) {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::DisconnectedNode,
                            message: format!(
                                "Node '{}' has no incoming edges",
                                node.name
                            ),
                            node_id: Some(node.id),
                        });
                    }

                    if !outgoing.get(&node.id).map(|v| !v.is_empty()).unwrap_or(false)
                        && node.node_type != NodeType::End
                    {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::DisconnectedNode,
                            message: format!(
                                "Node '{}' has no outgoing edges",
                                node.name
                            ),
                            node_id: Some(node.id),
                        });
                    }
                }
            }
        }

        // Check for orphaned nodes (not reachable from Start)
        let reachable = self.find_reachable_nodes(&outgoing);
        for node in &self.nodes {
            if !reachable.contains(&node.id) && node.node_type != NodeType::Start {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::UnreachableNode,
                    message: format!(
                        "Node '{}' is not reachable from Start node",
                        node.name
                    ),
                    node_id: Some(node.id),
                });
            }
        }

        errors
    }

    /// Find all nodes reachable from Start nodes
    fn find_reachable_nodes(&self, outgoing: &HashMap<Uuid, Vec<Uuid>>) -> HashSet<Uuid> {
        let mut reachable = HashSet::new();
        let mut stack = Vec::new();

        // Start from all Start nodes
        for node in &self.nodes {
            if node.node_type == NodeType::Start {
                stack.push(node.id);
                reachable.insert(node.id);
            }
        }

        // DFS traversal
        while let Some(node_id) = stack.pop() {
            if let Some(neighbors) = outgoing.get(&node_id) {
                for &neighbor in neighbors {
                    if !reachable.contains(&neighbor) {
                        reachable.insert(neighbor);
                        stack.push(neighbor);
                    }
                }
            }
        }

        reachable
    }

    /// Validate that workflow has no cycles (MANDATORY REQUIREMENT)
    fn validate_no_cycles(&self) -> Option<ValidationError> {
        let mut detector = CycleDetector::new();

        // Build graph
        for edge in &self.edges {
            detector.add_edge(edge.source_node_id, edge.target_node_id);
        }

        // Check for cycles
        if detector.has_cycle() {
            let cycles = detector.find_cycles();
            let cycle_info = if !cycles.is_empty() {
                format!(
                    " Found {} cycle(s). First cycle involves {} nodes.",
                    cycles.len(),
                    cycles[0].len()
                )
            } else {
                String::new()
            };

            return Some(ValidationError {
                error_type: ValidationErrorType::CycleDetected,
                message: format!(
                    "Workflow contains infinite loop(s).{}",
                    cycle_info
                ),
                node_id: None,
            });
        }

        None
    }

    /// Validate variable types throughout the workflow (MANDATORY REQUIREMENT)
    fn validate_types(&self) -> Vec<ValidationError> {
        let type_checker = TypeChecker::from_variables(&self.variables);
        let type_errors = type_checker.validate_workflow_nodes(&self.nodes);

        type_errors
            .into_iter()
            .map(|err| self.type_error_to_validation_error(err))
            .collect()
    }

    fn type_error_to_validation_error(&self, err: TypeError) -> ValidationError {
        ValidationError {
            error_type: ValidationErrorType::TypeMismatch,
            message: err.to_string(),
            node_id: None,
        }
    }

    /// Validate node-specific requirements
    fn validate_nodes(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for node in &self.nodes {
            // Validate node configuration based on type
            match node.node_type {
                NodeType::Decision => {
                    if !node.config.get("condition").is_some() {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: "Decision node must have a 'condition' in config".to_string(),
                            node_id: Some(node.id),
                        });
                    }
                }
                NodeType::Loop => {
                    if !node.config.get("condition").is_some()
                        && !node.config.get("max_iterations").is_some()
                    {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: "Loop node must have either 'condition' or 'max_iterations' in config".to_string(),
                            node_id: Some(node.id),
                        });
                    }

                    // Check for reasonable max_iterations
                    if let Some(max_iter) = node.config.get("max_iterations").and_then(|v| v.as_i64()) {
                        if max_iter <= 0 || max_iter > 10000 {
                            errors.push(ValidationError {
                                error_type: ValidationErrorType::InvalidConfiguration,
                                message: format!(
                                    "Loop max_iterations must be between 1 and 10000, got {}",
                                    max_iter
                                ),
                                node_id: Some(node.id),
                            });
                        }
                    }
                }
                NodeType::ApiCall => {
                    if !node.config.get("url").is_some() {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: "ApiCall node must have a 'url' in config".to_string(),
                            node_id: Some(node.id),
                        });
                    }
                }
                NodeType::Assignment => {
                    if !node.config.get("variable").is_some() {
                        errors.push(ValidationError {
                            error_type: ValidationErrorType::InvalidConfiguration,
                            message: "Assignment node must have a 'variable' in config".to_string(),
                            node_id: Some(node.id),
                        });
                    }
                }
                _ => {}
            }
        }

        errors
    }

    /// Validate edge-specific requirements
    fn validate_edges(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let node_ids: HashSet<Uuid> = self.nodes.iter().map(|n| n.id).collect();

        for edge in &self.edges {
            // Validate that source and target nodes exist
            if !node_ids.contains(&edge.source_node_id) {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::InvalidConfiguration,
                    message: format!(
                        "Edge references non-existent source node: {}",
                        edge.source_node_id
                    ),
                    node_id: Some(edge.source_node_id),
                });
            }

            if !node_ids.contains(&edge.target_node_id) {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::InvalidConfiguration,
                    message: format!(
                        "Edge references non-existent target node: {}",
                        edge.target_node_id
                    ),
                    node_id: Some(edge.target_node_id),
                });
            }

            // Check for self-loops (node connecting to itself)
            if edge.source_node_id == edge.target_node_id {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::InvalidConfiguration,
                    message: "Edge cannot connect node to itself (self-loop)".to_string(),
                    node_id: Some(edge.source_node_id),
                });
            }
        }

        errors
    }

    /// Generate warnings for potential issues
    fn generate_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        // Warn if workflow has many nodes
        if self.nodes.len() > 50 {
            warnings.push(format!(
                "Workflow has {} nodes. Consider breaking it into smaller workflows.",
                self.nodes.len()
            ));
        }

        // Warn if no variables defined
        if self.variables.is_empty() {
            warnings.push("Workflow has no variables defined.".to_string());
        }

        // Warn about deeply nested paths
        let max_depth = self.calculate_max_path_depth();
        if max_depth > 10 {
            warnings.push(format!(
                "Workflow has maximum path depth of {}. This may be hard to maintain.",
                max_depth
            ));
        }

        warnings
    }

    /// Calculate the maximum path depth in the workflow
    fn calculate_max_path_depth(&self) -> usize {
        let mut outgoing: HashMap<Uuid, Vec<Uuid>> = HashMap::new();

        for edge in &self.edges {
            outgoing
                .entry(edge.source_node_id)
                .or_insert_with(Vec::new)
                .push(edge.target_node_id);
        }

        let mut max_depth = 0;

        for node in &self.nodes {
            if node.node_type == NodeType::Start {
                let depth = self.dfs_depth(node.id, &outgoing, &mut HashSet::new());
                max_depth = max_depth.max(depth);
            }
        }

        max_depth
    }

    fn dfs_depth(
        &self,
        node_id: Uuid,
        outgoing: &HashMap<Uuid, Vec<Uuid>>,
        visited: &mut HashSet<Uuid>,
    ) -> usize {
        if visited.contains(&node_id) {
            return 0; // Avoid cycles in depth calculation
        }

        visited.insert(node_id);

        let mut max_child_depth = 0;
        if let Some(neighbors) = outgoing.get(&node_id) {
            for &neighbor in neighbors {
                let depth = self.dfs_depth(neighbor, outgoing, visited);
                max_child_depth = max_child_depth.max(depth);
            }
        }

        visited.remove(&node_id);

        1 + max_child_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_workflow() -> Workflow {
        Workflow {
            id: Uuid::new_v4(),
            name: "Test Workflow".to_string(),
            description: None,
            entity_id: None,
            trigger_type: shared::models::workflow::TriggerType::Manual,
            is_active: true,
            version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_valid_simple_workflow() {
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

        let validator = WorkflowValidator::new(
            workflow,
            vec![start_node, end_node],
            vec![edge],
            vec![],
        );

        let result = validator.validate();
        assert!(result.is_valid, "Simple workflow should be valid");
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_missing_start_node() {
        let workflow = create_test_workflow();

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

        let validator = WorkflowValidator::new(workflow, vec![end_node], vec![], vec![]);

        let result = validator.validate();
        assert!(!result.is_valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.error_type == ValidationErrorType::MissingStartNode));
    }

    #[test]
    fn test_cycle_detection() {
        let workflow = create_test_workflow();

        let node1 = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            node_type: NodeType::Task,
            name: "Task 1".to_string(),
            config: serde_json::json!({}),
            position_x: 0.0,
            position_y: 0.0,
            created_at: Utc::now(),
        };

        let node2 = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            node_type: NodeType::Task,
            name: "Task 2".to_string(),
            config: serde_json::json!({}),
            position_x: 100.0,
            position_y: 0.0,
            created_at: Utc::now(),
        };

        // Create cycle: node1 -> node2 -> node1
        let edge1 = WorkflowEdge {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            source_node_id: node1.id,
            target_node_id: node2.id,
            condition: None,
            order: 0,
            created_at: Utc::now(),
        };

        let edge2 = WorkflowEdge {
            id: Uuid::new_v4(),
            workflow_id: workflow.id,
            source_node_id: node2.id,
            target_node_id: node1.id,
            condition: None,
            order: 0,
            created_at: Utc::now(),
        };

        let validator = WorkflowValidator::new(
            workflow,
            vec![node1, node2],
            vec![edge1, edge2],
            vec![],
        );

        let result = validator.validate();
        assert!(!result.is_valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.error_type == ValidationErrorType::CycleDetected));
    }
}
