use crate::{
    dto::user::{GetUserPath, UserResponse},
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use breachlet_core::error::AppResult;
use breachlet_domain::user::repo::UserRepository;

pub async fn get_user<U>(
    State(state): State<AppState<U>>,
    Path(id): Path<String>,
) -> AppResult<Json<UserResponse>>
where
    U: UserRepository + Clone + 'static,
{
    // Parse path req as either Uuid or Email
    let id: GetUserPath = id.parse()?;

    // Query the user depending on the identifier type
    let user = match id {
        GetUserPath::Id(uuid) => state.user_service.get_user_by_id(uuid).await?,
        GetUserPath::Email(email) => state.user_service.get_user_by_email(email).await?,
    };

    // Respond with the found user
    Ok(Json(user))
}

pub fn build_user_handler<U>() -> Router<AppState<U>>
where
    U: UserRepository + Clone + 'static,
{
    Router::new().route("/{id}", get(get_user::<U>))
}
