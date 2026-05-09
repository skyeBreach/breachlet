pub mod db_config;
pub mod server_config;

use crate::constants::ENV_PREFIX;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use crate::config::{db_config::DatabaseConfig, server_config::ServerConfig};

// ================================================================================================================== //
// Definition

#[derive(Debug, Deserialize, Clone)]
pub struct BackendConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

// ================================================================================================================== //
// Implementation

impl BackendConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            //
            .add_source(File::with_name("config/default"))
            //
            .add_source(
                Environment::with_prefix(ENV_PREFIX)
                    .prefix_separator("__")
                    .separator("_"),
            )
            //
            .add_source(Environment::default().try_parsing(true))
            .build()?;

        config.try_deserialize()
    }
}

// ================================================================================================================== //
