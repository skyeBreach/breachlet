use async_trait::async_trait;
use breachlet_core::error::AppResult;
use breachlet_domain::auth::{entity::UserAuthMethod, repo::AuthRepository};
use sqlx::PgPool;

use crate::models::auth_model::UserAuthMethodModel;

#[derive(Clone)]
pub struct PgAuthRepository {
    pool: PgPool,
}

#[async_trait]
impl AuthRepository for PgAuthRepository {
    async fn find_user_auth_method_by_id(
        &self,
        id: uuid::Uuid,
    ) -> AppResult<Option<UserAuthMethod>> {
        let result: Option<UserAuthMethodModel> =
            sqlx::query_as::<_, UserAuthMethodModel>(r#"SELECT id,user_id, auth_provider, credential, created_at, updated_at FROM user_auth_methods WHERE id = $1"#)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(result.map(UserAuthMethodModel::into_entity))
    }
}
