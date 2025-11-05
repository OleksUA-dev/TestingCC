use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub entity_id: Option<Uuid>,
    pub trigger_type: TriggerType,
    pub is_active: bool,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Workflow trigger types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TriggerType {
    /// Manual trigger
    Manual,
    /// On record create
    OnCreate,
    /// On record update
    OnUpdate,
    /// On record delete
    OnDelete,
    /// Scheduled (cron)
    Scheduled,
}

impl std::fmt::Display for TriggerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TriggerType::Manual => write!(f, "manual"),
            TriggerType::OnCreate => write!(f, "on_create"),
            TriggerType::OnUpdate => write!(f, "on_update"),
            TriggerType::OnDelete => write!(f, "on_delete"),
            TriggerType::Scheduled => write!(f, "scheduled"),
        }
    }
}

/// Workflow node (step/block in the flow)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub node_type: NodeType,
    pub name: String,
    pub position_x: f64,
    pub position_y: f64,
    pub config: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Node types (logical blocks)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    /// Start node
    Start,
    /// End node
    End,
    /// Execute task/action
    Task,
    /// Conditional branching (if/else)
    Decision,
    /// Parallel execution (fork)
    Fork,
    /// Wait for parallel branches (join)
    Join,
    /// Assign variable
    Assignment,
    /// Loop iteration
    Loop,
    /// Call API
    ApiCall,
    /// Send email/notification
    Notification,
    /// Wait/delay
    Wait,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeType::Start => write!(f, "start"),
            NodeType::End => write!(f, "end"),
            NodeType::Task => write!(f, "task"),
            NodeType::Decision => write!(f, "decision"),
            NodeType::Fork => write!(f, "fork"),
            NodeType::Join => write!(f, "join"),
            NodeType::Assignment => write!(f, "assignment"),
            NodeType::Loop => write!(f, "loop"),
            NodeType::ApiCall => write!(f, "api_call"),
            NodeType::Notification => write!(f, "notification"),
            NodeType::Wait => write!(f, "wait"),
        }
    }
}

/// Workflow edge (connection between nodes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub source_node_id: Uuid,
    pub target_node_id: Uuid,
    pub condition: Option<String>,
    pub label: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Workflow variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVariable {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub name: String,
    pub variable_type: VariableType,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Variable types for type checking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    String,
    Integer,
    Float,
    Boolean,
    Date,
    DateTime,
    Array,
    Object,
    Any,
}

impl std::fmt::Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::String => write!(f, "string"),
            VariableType::Integer => write!(f, "integer"),
            VariableType::Float => write!(f, "float"),
            VariableType::Boolean => write!(f, "boolean"),
            VariableType::Date => write!(f, "date"),
            VariableType::DateTime => write!(f, "date_time"),
            VariableType::Array => write!(f, "array"),
            VariableType::Object => write!(f, "object"),
            VariableType::Any => write!(f, "any"),
        }
    }
}

/// Workflow execution instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub status: ExecutionStatus,
    pub context: serde_json::Value,
    pub current_node_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionStatus::Pending => write!(f, "pending"),
            ExecutionStatus::Running => write!(f, "running"),
            ExecutionStatus::Completed => write!(f, "completed"),
            ExecutionStatus::Failed => write!(f, "failed"),
            ExecutionStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Request to create workflow
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateWorkflowRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub entity_id: Option<Uuid>,
    pub trigger_type: TriggerType,
}

/// Request to add node to workflow
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateNodeRequest {
    pub workflow_id: Uuid,
    pub node_type: NodeType,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub position_x: f64,
    pub position_y: f64,
    pub config: serde_json::Value,
}

/// Request to create edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEdgeRequest {
    pub workflow_id: Uuid,
    pub source_node_id: Uuid,
    pub target_node_id: Uuid,
    pub condition: Option<String>,
    pub label: Option<String>,
}

/// Workflow validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

/// Validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub message: String,
    pub node_id: Option<Uuid>,
    pub edge_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationErrorType {
    MissingStartNode,
    MissingEndNode,
    MultipleStartNodes,
    DisconnectedNode,
    UnreachableNode,
    CycleDetected,
    CyclicDependency,
    TypeMismatch,
    InvalidCondition,
    InvalidConfiguration,
    MissingRequiredField,
}
