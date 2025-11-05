use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Configuration(String),
}

pub type CoreResult<T> = Result<T, CoreError>;

impl CoreError {
    pub fn status_code(&self) -> u16 {
        match self {
            CoreError::NotFound(_) => 404,
            CoreError::InvalidInput(_) => 400,
            CoreError::Validation(_) => 400,
            CoreError::Unauthorized(_) => 401,
            CoreError::Forbidden(_) => 403,
            CoreError::Database(_) => 500,
            CoreError::Internal(_) => 500,
            CoreError::Configuration(_) => 500,
        }
    }
}
