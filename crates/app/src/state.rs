use breachlet_core::{config::db_config::DatabaseConfig, error::AppResult};
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub async fn init(config: DatabaseConfig) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect(&config.url())
            .await?;

        Ok(Self { db_pool: pool })
    }
}
