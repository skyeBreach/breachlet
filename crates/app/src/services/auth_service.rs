use crate::dto::auth::UserAuthMethodResponse;
use breachlet_core::error::{AppError, AppResult};
use breachlet_domain::{auth::repo::AuthRepository, user::repo::UserRepository};
use sqlx::types::Uuid;
use std::sync::Arc;

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    auth_repo: Arc<dyn AuthRepository>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        auth_repo: Arc<dyn AuthRepository>,
    ) -> Self {
        Self { user_repo, auth_repo }
    }

    pub async fn get_auth_method_by_id(
        &self,
        id: Uuid,
    ) -> AppResult<UserAuthMethodResponse> {
        // Get user via users repo
        let user_auth_method = self
            .auth_repo
            .find_user_auth_method_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Could not find user with id {}", id)))?;

        // Convert found user to response dto
        let res = UserAuthMethodResponse::from(user_auth_method);

        Ok(res)
    }
}
