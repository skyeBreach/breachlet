use breachlet_core::error::AppResult;
use uuid::Uuid;

use crate::user::entity::User;

pub trait UserRepository: Send + Sync {
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = AppResult<Option<User>>> + Send;

    fn find_by_email(
        &self,
        email: String,
    ) -> impl Future<Output = AppResult<Option<User>>> + Send;
}
