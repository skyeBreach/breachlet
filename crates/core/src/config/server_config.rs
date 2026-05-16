use serde::Deserialize;

// =================================================================================================================/ //
// Definition

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

// ================================================================================================================== //
// Implementation

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

// ================================================================================================================== //
// Default Values

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}

// ================================================================================================================== //
