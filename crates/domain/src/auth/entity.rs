use std::net::IpAddr;

use crate::auth::value_object::{AuthProvider, UserSessionStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthMethod {
    pub id: Uuid,
    pub user_id: Uuid,
    pub auth_provider: AuthProvider,
    pub credential: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserAuthMethod {
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        auth_provider: AuthProvider,
        credential: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            auth_provider,
            credential,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub session_id: Uuid,
    pub user_id: Uuid,

    pub status: UserSessionStatus,
    pub expire_at: DateTime<Utc>,
    pub revoked_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,

    pub ip_address: IpAddr,
    pub user_agent: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserSession {
    pub fn new(
        session_id: Uuid,
        user_id: Uuid,

        status: UserSessionStatus,
        expire_at: DateTime<Utc>,
        revoked_at: DateTime<Utc>,
        last_activity_at: DateTime<Utc>,

        ip_address: IpAddr,
        user_agent: String,

        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            session_id,
            user_id,
            status,
            expire_at,
            revoked_at,
            ip_address,
            user_agent,
            last_activity_at,
            created_at,
            updated_at,
        }
    }
}
