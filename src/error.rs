use axum::response::{IntoResponse, Response};
use http::StatusCode;

#[derive(Debug)]
pub enum AppError {
    FileIO(std::io::Error),
    InvalidParams(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::FileIO(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("File operation failed: {}", e),
            ),
            Self::InvalidParams(msg) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid parameters: {}", msg),
            ),
        }
        .into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::FileIO(e)
    }
}
