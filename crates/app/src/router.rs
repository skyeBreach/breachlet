use axum::{Router, routing::get};
use breachlet_core::error::AppResult;
use breachlet_domain::user::repo::UserRepository;

use crate::{
    handler::{health::health, user::build_user_handler},
    state::AppState,
};

pub async fn build_router(app_state: AppState) -> AppResult<Router> {
    Ok(Router::new()
        .route("/health", get(health))
        .nest("/api", api_routes())
        .with_state(app_state))
}

fn api_routes() -> Router<AppState> {
    Router::new().nest("/user", build_user_handler())
}
