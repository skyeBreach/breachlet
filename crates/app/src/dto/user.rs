use serde::{Deserialize, Serialize};
use sqlx::types::{
    chrono::{DateTime, Utc},
    uuid,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub last_login: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
