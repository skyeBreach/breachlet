use crate::dto::health::{HealthResponse, HealthStatus};
use axum::{Json, http::StatusCode, response::IntoResponse};

pub async fn health() -> impl IntoResponse {
    let body = HealthResponse {
        status: HealthStatus::Healthy,
        version: env!("CARGO_PKG_VERSION"),
    };

    (StatusCode::OK, Json(body))
}
