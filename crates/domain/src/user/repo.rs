use async_trait::async_trait;
use breachlet_core::error::AppResult;
use uuid::Uuid;

use crate::user::entity::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> AppResult<Option<User>>;

    async fn find_by_email(
        &self,
        email: String,
    ) -> AppResult<Option<User>>;
}
