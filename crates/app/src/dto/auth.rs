use breachlet_domain::auth::{entity::UserAuthMethod, value_object::AuthProvider};
use serde::{Deserialize, Serialize};
use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthMethodResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub auth_provider: AuthProvider,
    pub credential: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserAuthMethod> for UserAuthMethodResponse {
    fn from(value: UserAuthMethod) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            auth_provider: value.auth_provider,
            credential: value.credential,

            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
