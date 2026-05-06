use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct UserModel {
    // User Identification
    pub id: Uuid,
    pub username: String,
    pub email: String,

    // Authentication
    pub password_hash: String,

    // Metadata
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
