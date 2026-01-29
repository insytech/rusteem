use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_msg, detail) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "Not Found", Some(msg.clone())),
            AppError::Validation(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Validation Error", Some(msg.clone()))
            }
            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, "Unauthorized", Some(msg.clone()))
            }
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "Forbidden", Some(msg.clone())),
            AppError::Database(err) => {
                tracing::error!(error = %err, "Database error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", None)
            }
            AppError::Internal(msg) => {
                tracing::error!(error = %msg, "Internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", None)
            }
        };

        let body = ErrorResponse {
            error: error_msg.to_string(),
            detail,
        };

        (status, axum::Json(body)).into_response()
    }
}
