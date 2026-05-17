use std::net::IpAddr;

use breachlet_domain::auth::{
    entity::{UserAuthMethod, UserSession},
    value_object::{AuthProvider, UserSessionStatus},
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

// ================================================================================================================== //
// Auth Providers

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "auth_provider", rename_all = "lowercase")]
pub enum AuthProviderModel {
    Password,
}

impl AuthProviderModel {
    pub fn into_entity(self) -> AuthProvider {
        match self {
            Self::Password => AuthProvider::Password,
        }
    }
}

// ================================================================================================================== //
// User Authentication Methods

#[derive(Debug, sqlx::FromRow)]
pub struct UserAuthMethodModel {
    // Identification
    pub id: Uuid,
    pub user_id: Uuid,

    // Method
    pub auth_provider: AuthProviderModel,
    pub credential: String,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserAuthMethodModel {
    pub fn into_entity(self) -> UserAuthMethod {
        UserAuthMethod {
            id: self.id,
            user_id: self.user_id,
            auth_provider: self.auth_provider.into_entity(),
            credential: self.credential,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ================================================================================================================== //
// Session Status

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "user_session_status", rename_all = "lowercase")]
pub enum UserSessionStatusModel {
    Active,
    Expired,
    Revoked,
}

impl UserSessionStatusModel {
    pub fn into_value_object(self) -> UserSessionStatus {
        match self {
            Self::Active => UserSessionStatus::Active,
            Self::Expired => UserSessionStatus::Expired,
            Self::Revoked => UserSessionStatus::Revoked,
        }
    }
}

// ================================================================================================================== //
// User Sessions

#[derive(Debug, sqlx::FromRow)]
pub struct UserSessionModel {
    // Identification
    pub session_id: Uuid,
    pub user_id: Uuid,

    // Session Status
    pub status: UserSessionStatus,
    pub expire_at: DateTime<Utc>,
    pub revoked_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,

    // Client Identification
    pub ip_address: IpAddr,
    pub user_agent: String,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ================================================================================================================== //
