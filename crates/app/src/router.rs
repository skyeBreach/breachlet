use axum::{Router, extract::State, routing::get};
use breachlet_core::error::AppResult;
use breachlet_domain::user::repo::UserRepository;

use crate::{
    handler::{health::health, user::build_user_handler},
    state::AppState,
};

pub async fn build_router<U>(app_state: AppState<U>) -> AppResult<Router>
where
    U: UserRepository + Clone + 'static,
{
    Ok(Router::new()
        .route("/health", get(health))
        .nest("/api", api_routes::<U>())
        .with_state(app_state))
}

fn api_routes<U>() -> Router<AppState<U>>
where
    U: UserRepository + Clone + 'static,
{
    Router::new().nest("/users", build_user_handler::<U>())
}
