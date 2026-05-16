use std::result;

use breachlet_core::error::AppResult;
use breachlet_domain::user::{entity::User, repo::UserRepository};
use sqlx::PgPool;
use tracing::info;

use crate::models::user_model::UserModel;

#[derive(Clone)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    async fn find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"SELECT id, username, email, last_login, created_at, updated_at FROM users WHERE id = $1"#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(UserModel::into_entity))
    }

    async fn find_by_email(
        &self,
        email: String,
    ) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, UserModel>(
            r#"SELECT id, username, email, last_login, created_at, updated_at FROM users WHERE email = $1"#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(UserModel::into_entity))
    }
}
