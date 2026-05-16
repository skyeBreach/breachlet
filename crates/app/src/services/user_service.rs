use std::sync::Arc;

use breachlet_core::error::{AppError, AppResult};
use breachlet_domain::user::repo::UserRepository;
use sqlx::types::Uuid;

use crate::dto::user::UserResponse;

pub struct UserService<U>
where
    U: UserRepository,
{
    user_repo: Arc<U>,
}

impl<U> UserService<U>
where
    U: UserRepository,
{
    pub fn new(user_repo: Arc<U>) -> Self {
        Self { user_repo }
    }

    pub async fn get_user_by_id(
        &self,
        id: Uuid,
    ) -> AppResult<UserResponse> {
        // Get user via users repo
        let user = self
            .user_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Could not find user with id {}", id)))?;

        // Convert found user to response dto
        let res = UserResponse::from(user);

        Ok(res)
    }
}
