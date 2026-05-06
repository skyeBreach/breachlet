-- migrations/0001_initial_schema.up.sql
-- ================================================================================================================== --
-- User Management

CREATE TABLE users (
    -- User Identification
    id              uuid
                    PRIMARY KEY
                    DEFAULT gen_random_uuid(),
    username        VARCHAR(64)
                    NOT NULL
                    UNIQUE,
    email           VARCHAR(255)
                    NOT NULL,

    -- Authentication
    password_hash   VARCHAR(255)
                    NOT NULL,

    -- Metadata
    last_login      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ
                    NOT NULL
                    DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMPTZ
                    NOT NULL
                    DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);

COMMENT ON TABLE users IS 'User Accounts';

-- ================================================================================================================== --
-- Authentication

CREATE TYPE user_session_status AS ENUM (
    'active',
    'expired',
    'revoked'
);

CREATE TABLE user_sessions (
    -- Core
    session_id          UUID
                        PRIMARY KEY 
                        DEFAULT gen_random_uuid(),
    user_id             UUID
                        NOT NULL
                        REFERENCES users(id),
    expire_at           TIMESTAMPTZ
                        NOT NULL,

    -- Session Status
    status              user_session_status,
    revoked_at          TIMESTAMPTZ,
    last_activity_at    TIMESTAMPTZ,

    -- Client Identification
    ip_address          inet
                        NOT NULL,
    user_agent          TEXT
                        NOT NULL,

    -- Metadata
    created_at          TIMESTAMPTZ
                        NOT NULL
                        DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMPTZ
                        NOT NULL
                        DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_expire_at ON user_sessions(expire_at);
CREATE INDEX idx_user_sessions_status ON user_sessions(status);
CREATE INDEX idx_user_sessions_ip_address ON user_sessions(ip_address);

-- ================================================================================================================== --
