use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String), // e.g., Resource already exists

    #[error("WebSocket error: {0}")]
    WebSocketError(#[from] axum::Error), // If needed for WS errors propagating up

    // Add other specific error types as needed
}

// Implement IntoResponse for AppError to automatically convert errors to HTTP responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::ConfigError(msg) => {
                tracing::error!("Configuration error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::WebSocketError(e) => {
                 tracing::error!("WebSocket setup error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "WebSocket error".to_string())
            }
            // Handle other variants
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

// Allow converting other errors (like io::Error) into AppError if needed
// impl From<std::io::Error> for AppError {
//     fn from(err: std::io::Error) -> Self {
//         AppError::InternalServerError(err.to_string())
//     }
// }
