use axum::response::IntoResponse;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum AppError  {
    #[error("File I/O error: {0}")]
    FileIO(#[from] std::io::Error),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Index out of bounds: {0}")]
    IndexError(String),

    #[error("Security violaation: {0}")]
    SecurityViolation(String),

    #[error("File too large (>1MB))")]
    FileSizeExceeded,
}

impl From<std::num::TryFromIntError> for AppError {
    fn from(error: std::num::TryFromIntError) -> Self {
        Self::IndexError(error.to_string())
    }
} 

impl IntoResponse for AppError  {
    fn into_response(self) -> axum::response::Response {
        let body = format!("Error: {}", self);
        body.into_response() // Convert the error message to a response body
    }
    
}
