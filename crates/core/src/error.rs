use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;
use thiserror::Error;

// ================================================================================================================== //
// Definition

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Body Failed To Validate: {0}")]
    Validation(#[from] garde::Report),
    #[error("Config Parse Error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Database Error: {0}")]
    Database(#[from] sqlx::error::Error),
    #[error("Internal Server Error: {0}")]
    Internal(#[from] anyhow::Error),
}

// ================================================================================================================== //
// HTTP Response

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Build the error response details based on its type
        let (status, code, message) = match &self {
            Self::Validation(inner) => (StatusCode::BAD_REQUEST, "VALIDATION_FAILURE", inner.to_string()),
            Self::Config(inner) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "CONFIG_FAILED_TO_PARSE",
                inner.to_string(),
            ),
            Self::Database(inner) => {
                tracing::error!("Stacktrace: {}", inner.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Internal Server Error".to_string(),
                )
            }
            Self::Internal(inner) => {
                tracing::error!("Stacktrace: {}", inner.backtrace());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Internal Server Error".to_string(),
                )
            }
        };

        // Construct json response body from error details
        let body = Json(json!({
            "success": false,
            "error": {
                "code": code,
                "message": message
            }
        }));

        // Convert the response body into full response
        (status, body).into_response()
    }
}

// ================================================================================================================== //
// App Result

pub type AppResult<T> = Result<T, AppError>;

// ================================================================================================================== //
