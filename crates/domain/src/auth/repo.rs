use breachlet_core::error::AppResult;
use uuid::Uuid;

use crate::auth::entity::UserAuthMethod;

pub trait AuthRepository: Send + Sync {
    fn get_user_auth_method_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = AppResult<Option<UserAuthMethod>>> + Send;
}
