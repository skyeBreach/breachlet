use crate::{dto::user::UserResponse, state::AppState};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use breachlet_core::error::AppResult;
use breachlet_domain::user::repo::UserRepository;
use sqlx::types::Uuid;

pub async fn get_user<U>(
    State(state): State<AppState<U>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<UserResponse>>
where
    U: UserRepository + Clone + 'static,
{
    // TODO: Query User
    //
    let user = state.user_service.get_user_by_id(id).await?;

    // TODO: Map User into response DTO
    //
    Ok(Json(user))
}

pub fn build_user_handler<U>() -> Router<AppState<U>>
where
    U: UserRepository + Clone + 'static,
{
    Router::new().route("/{id}", get(get_user::<U>))
}
