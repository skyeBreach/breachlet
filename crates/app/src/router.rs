use axum::{Router, routing::get};
use breachlet_core::error::AppResult;

use crate::{handler::health::health, state::AppState};

pub async fn build_app_router(app_state: AppState) -> AppResult<Router> {
    Ok(Router::new().route("/health", get(health).with_state(app_state)))
}
