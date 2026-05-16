use breachlet_domain::user::entity::User;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct UserModel {
    // User Identification
    pub id: Uuid,
    pub username: String,
    pub email: String,

    // Metadata
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserModel {
    pub fn into_entity(self) -> User {
        User {
            id: self.id,
            username: self.username,
            email: self.email,
            last_login: self.last_login,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
