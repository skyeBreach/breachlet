use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Body Failed: {0}")]
    Validation(#[from] garde::Report),
    #[error("Internal Server Error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Build the error response details based on its type
        let (status, code, message) = match &self {
            Self::Validation(inner) => (StatusCode::BAD_REQUEST, "VALIDATION_FAILURE", inner.to_string()),
            Self::Internal(inner) => {
                tracing::error!("Stacktrace: {}", inner.backtrace());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Internal Server Error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "success": false,
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
