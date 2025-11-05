use std::collections::HashMap;
use uuid::Uuid;

use shared::models::workflow::{VariableType, WorkflowNode, WorkflowVariable, NodeType};
use shared::CoreResult;

/// Type error that can occur during type checking
#[derive(Debug, Clone)]
pub enum TypeError {
    UndeclaredVariable(String),
    TypeMismatch {
        variable: String,
        expected: VariableType,
        found: VariableType,
    },
    InvalidOperation {
        operation: String,
        types: Vec<VariableType>,
    },
    InvalidCondition {
        condition: String,
        reason: String,
    },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UndeclaredVariable(var) => {
                write!(f, "Undeclared variable: {}", var)
            }
            TypeError::TypeMismatch {
                variable,
                expected,
                found,
            } => {
                write!(
                    f,
                    "Type mismatch for variable '{}': expected {:?}, found {:?}",
                    variable, expected, found
                )
            }
            TypeError::InvalidOperation { operation, types } => {
                write!(
                    f,
                    "Invalid operation '{}' for types: {:?}",
                    operation, types
                )
            }
            TypeError::InvalidCondition { condition, reason } => {
                write!(
                    f,
                    "Invalid condition '{}': {}",
                    condition, reason
                )
            }
        }
    }
}

impl std::error::Error for TypeError {}

/// Type checker for workflow variables and operations
pub struct TypeChecker {
    variables: HashMap<String, VariableType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Initialize type checker with workflow variables
    pub fn from_variables(variables: &[WorkflowVariable]) -> Self {
        let mut checker = Self::new();
        for var in variables {
            checker.declare_variable(var.name.clone(), var.variable_type.clone());
        }
        checker
    }

    /// Declare a variable with its type
    pub fn declare_variable(&mut self, name: String, var_type: VariableType) {
        self.variables.insert(name, var_type);
    }

    /// Get the type of a variable
    pub fn get_variable_type(&self, name: &str) -> Option<&VariableType> {
        self.variables.get(name)
    }

    /// Check if a variable exists
    pub fn has_variable(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Check if an assignment is type-compatible
    pub fn check_assignment(
        &self,
        var_name: &str,
        value_type: VariableType,
    ) -> Result<(), TypeError> {
        match self.variables.get(var_name) {
            None => Err(TypeError::UndeclaredVariable(var_name.to_string())),
            Some(expected_type) => {
                if self.is_compatible(expected_type, &value_type) {
                    Ok(())
                } else {
                    Err(TypeError::TypeMismatch {
                        variable: var_name.to_string(),
                        expected: expected_type.clone(),
                        found: value_type,
                    })
                }
            }
        }
    }

    /// Check if two types are compatible
    pub fn is_compatible(&self, target: &VariableType, source: &VariableType) -> bool {
        // Any type can accept any value
        if matches!(target, VariableType::Any) {
            return true;
        }

        // Same types are compatible
        if target == source {
            return true;
        }

        // Integer can be assigned to Float
        if matches!(target, VariableType::Float) && matches!(source, VariableType::Integer) {
            return true;
        }

        false
    }

    /// Validate a condition expression (simplified validation)
    pub fn check_condition(&self, condition: &str) -> Result<(), TypeError> {
        if condition.trim().is_empty() {
            return Err(TypeError::InvalidCondition {
                condition: condition.to_string(),
                reason: "Condition cannot be empty".to_string(),
            });
        }

        // Basic validation: check if referenced variables exist
        // In a real implementation, this would parse the expression
        let var_pattern = regex::Regex::new(r"\$\{(\w+)\}").unwrap();

        for cap in var_pattern.captures_iter(condition) {
            let var_name = &cap[1];
            if !self.has_variable(var_name) {
                return Err(TypeError::InvalidCondition {
                    condition: condition.to_string(),
                    reason: format!("Unknown variable: {}", var_name),
                });
            }
        }

        Ok(())
    }

    /// Check a node's type validity
    pub fn check_node(&self, node: &WorkflowNode) -> Result<(), TypeError> {
        match node.node_type {
            NodeType::Assignment => self.check_assignment_node(node),
            NodeType::Decision => self.check_decision_node(node),
            NodeType::Loop => self.check_loop_node(node),
            NodeType::ApiCall => self.check_api_call_node(node),
            _ => Ok(()), // Other node types don't need type checking
        }
    }

    fn check_assignment_node(&self, node: &WorkflowNode) -> Result<(), TypeError> {
        // Extract variable and value_type from config
        let config = &node.config;

        if let (Some(var_name), Some(value_type_str)) = (
            config.get("variable").and_then(|v| v.as_str()),
            config.get("value_type").and_then(|v| v.as_str()),
        ) {
            let value_type = self.parse_variable_type(value_type_str)?;
            self.check_assignment(var_name, value_type)?;
        }

        Ok(())
    }

    fn check_decision_node(&self, node: &WorkflowNode) -> Result<(), TypeError> {
        let config = &node.config;

        if let Some(condition) = config.get("condition").and_then(|v| v.as_str()) {
            self.check_condition(condition)?;
        }

        Ok(())
    }

    fn check_loop_node(&self, node: &WorkflowNode) -> Result<(), TypeError> {
        let config = &node.config;

        // Validate loop condition if present
        if let Some(condition) = config.get("condition").and_then(|v| v.as_str()) {
            self.check_condition(condition)?;
        }

        // Validate iteration variable type if present
        if let (Some(var_name), Some(value_type_str)) = (
            config.get("iterator_variable").and_then(|v| v.as_str()),
            config.get("value_type").and_then(|v| v.as_str()),
        ) {
            let value_type = self.parse_variable_type(value_type_str)?;

            if let Some(expected_type) = self.get_variable_type(var_name) {
                if !self.is_compatible(expected_type, &value_type) {
                    return Err(TypeError::TypeMismatch {
                        variable: var_name.to_string(),
                        expected: expected_type.clone(),
                        found: value_type,
                    });
                }
            }
        }

        Ok(())
    }

    fn check_api_call_node(&self, node: &WorkflowNode) -> Result<(), TypeError> {
        let config = &node.config;

        // Validate input parameters
        if let Some(params) = config.get("parameters").and_then(|v| v.as_object()) {
            for (param_name, param_value) in params {
                if let Some(var_name) = param_value.as_str() {
                    // Check if variable is a reference like ${variable}
                    if var_name.starts_with("${") && var_name.ends_with("}") {
                        let actual_var = &var_name[2..var_name.len() - 1];
                        if !self.has_variable(actual_var) {
                            return Err(TypeError::UndeclaredVariable(
                                actual_var.to_string(),
                            ));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_variable_type(&self, type_str: &str) -> Result<VariableType, TypeError> {
        match type_str.to_lowercase().as_str() {
            "string" => Ok(VariableType::String),
            "integer" | "int" => Ok(VariableType::Integer),
            "float" | "number" => Ok(VariableType::Float),
            "boolean" | "bool" => Ok(VariableType::Boolean),
            "date" => Ok(VariableType::Date),
            "datetime" => Ok(VariableType::DateTime),
            "array" => Ok(VariableType::Array),
            "object" => Ok(VariableType::Object),
            "any" => Ok(VariableType::Any),
            _ => Err(TypeError::InvalidOperation {
                operation: "parse_type".to_string(),
                types: vec![],
            }),
        }
    }

    /// Validate all nodes in a workflow for type correctness
    pub fn validate_workflow_nodes(&self, nodes: &[WorkflowNode]) -> Vec<TypeError> {
        let mut errors = Vec::new();

        for node in nodes {
            if let Err(err) = self.check_node(node) {
                errors.push(err);
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_declare_and_get_variable() {
        let mut checker = TypeChecker::new();
        checker.declare_variable("name".to_string(), VariableType::String);

        assert!(checker.has_variable("name"));
        assert_eq!(
            checker.get_variable_type("name"),
            Some(&VariableType::String)
        );
    }

    #[test]
    fn test_assignment_valid() {
        let mut checker = TypeChecker::new();
        checker.declare_variable("count".to_string(), VariableType::Integer);

        let result = checker.check_assignment("count", VariableType::Integer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assignment_type_mismatch() {
        let mut checker = TypeChecker::new();
        checker.declare_variable("count".to_string(), VariableType::Integer);

        let result = checker.check_assignment("count", VariableType::String);
        assert!(result.is_err());
    }

    #[test]
    fn test_assignment_undeclared_variable() {
        let checker = TypeChecker::new();

        let result = checker.check_assignment("unknown", VariableType::String);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TypeError::UndeclaredVariable(_)));
    }

    #[test]
    fn test_type_compatibility_any() {
        let checker = TypeChecker::new();

        assert!(checker.is_compatible(&VariableType::Any, &VariableType::String));
        assert!(checker.is_compatible(&VariableType::Any, &VariableType::Integer));
    }

    #[test]
    fn test_type_compatibility_numeric() {
        let checker = TypeChecker::new();

        // Integer can be assigned to Float
        assert!(checker.is_compatible(&VariableType::Float, &VariableType::Integer));

        // But not vice versa
        assert!(!checker.is_compatible(&VariableType::Integer, &VariableType::Float));
    }

    #[test]
    fn test_condition_validation() {
        let mut checker = TypeChecker::new();
        checker.declare_variable("age".to_string(), VariableType::Integer);

        // Valid condition
        let result = checker.check_condition("${age} > 18");
        assert!(result.is_ok());

        // Invalid: unknown variable
        let result = checker.check_condition("${unknown} > 18");
        assert!(result.is_err());

        // Invalid: empty condition
        let result = checker.check_condition("");
        assert!(result.is_err());
    }

    #[test]
    fn test_decision_node_validation() {
        let mut checker = TypeChecker::new();
        checker.declare_variable("status".to_string(), VariableType::String);

        let node = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: Uuid::new_v4(),
            node_type: NodeType::Decision,
            name: "Check Status".to_string(),
            config: json!({
                "condition": "${status} == 'active'"
            }),
            position_x: 0.0,
            position_y: 0.0,
            created_at: chrono::Utc::now(),
        };

        let result = checker.check_node(&node);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assignment_node_validation() {
        let mut checker = TypeChecker::new();
        checker.declare_variable("total".to_string(), VariableType::Float);

        let node = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: Uuid::new_v4(),
            node_type: NodeType::Assignment,
            name: "Set Total".to_string(),
            config: json!({
                "variable": "total",
                "value_type": "float"
            }),
            position_x: 0.0,
            position_y: 0.0,
            created_at: chrono::Utc::now(),
        };

        let result = checker.check_node(&node);
        assert!(result.is_ok());
    }

    #[test]
    fn test_api_call_node_validation() {
        let mut checker = TypeChecker::new();
        checker.declare_variable("user_id".to_string(), VariableType::String);

        let node = WorkflowNode {
            id: Uuid::new_v4(),
            workflow_id: Uuid::new_v4(),
            node_type: NodeType::ApiCall,
            name: "Fetch User".to_string(),
            config: json!({
                "url": "https://api.example.com/users",
                "parameters": {
                    "id": "${user_id}"
                }
            }),
            position_x: 0.0,
            position_y: 0.0,
            created_at: chrono::Utc::now(),
        };

        let result = checker.check_node(&node);
        assert!(result.is_ok());
    }
}
