use serde::Deserialize;

use crate::error::AppResult;

#[derive(Debug, Deserialize, Clone)]
pub struct BackendConfig {}

impl BackendConfig {
    pub fn load() -> AppResult<Self> {
        Ok(Self {})
    }
}
