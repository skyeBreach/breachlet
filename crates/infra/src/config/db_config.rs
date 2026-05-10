use serde::Deserialize;

// ================================================================================================================== //
// Definition

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub address: String,
    pub port: String,
    pub database: String,
    pub user: String,
    pub password: String,

    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
}

// ================================================================================================================== //
// Implementation

impl DatabaseConfig {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            &self.user, self.password, &self.address, &self.port, &self.database
        )
    }
}

// ================================================================================================================== //
// Defaults

fn default_max_connections() -> u32 {
    20
}

fn default_min_connections() -> u32 {
    5
}

// ================================================================================================================== //
