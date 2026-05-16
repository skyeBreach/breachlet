use breachlet_core::error::{AppError, AppResult};
use breachlet_domain::{user::entity::User, value_object::email::Email};

use serde::{Deserialize, Serialize};
use sqlx::types::{
    Uuid,
    chrono::{DateTime, Utc},
    uuid,
};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,

            last_login: value.last_login,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Clone, Deserialize)]
pub enum GetUserPath {
    Id(Uuid),
    Email(Email),
}

impl FromStr for GetUserPath {
    type Err = AppError;

    fn from_str(s: &str) -> AppResult<Self> {
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Ok(Self::Id(uuid));
        }

        if let Ok(email) = Email::parse_str(s) {
            return Ok(Self::Email(email));
        }

        let mut report = garde::Report::new();
        report.append(
            garde::Path::new("id"),
            garde::Error::new("Must be a valid UUID or email address"),
        );

        Err(AppError::Validation(report))
    }
}
