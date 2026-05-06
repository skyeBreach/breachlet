-- migrations/0002_populate_initial_schema.up.sql
-- ================================================================================================================== --
-- User Management

INSERT INTO users (
        username,
        email,
        password_hash
    )
VALUES (
    'skyeBreach',
    'skyebenson@hotmail.com',
    '$2a$12$axkx88bXYQ8RqXUK.wgjoeybNMTsIFQIBO3PIe1CsAbTqrxkkdgwS'
);

-- ================================================================================================================== --
