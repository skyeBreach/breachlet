use async_trait::async_trait;
use breachlet_core::error::AppResult;
use uuid::Uuid;

use crate::auth::entity::UserAuthMethod;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_user_auth_method_by_id(
        &self,
        id: Uuid,
    ) -> AppResult<Option<UserAuthMethod>>;
}
