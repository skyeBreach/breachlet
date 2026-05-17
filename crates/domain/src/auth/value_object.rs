use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserSessionStatus {
    Active,
    Expired,
    Revoked,
}
