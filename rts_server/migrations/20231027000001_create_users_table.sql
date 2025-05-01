-- Create Users Table

-- Consider using UUIDs for primary keys in distributed systems
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    -- id BIGSERIAL PRIMARY KEY, -- Auto-incrementing integer (simple)
    id BIGSERIAL PRIMARY KEY, -- Or use UUID: id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ
);

-- Add indexes for common lookups
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);

-- Add other tables: players, entities, map_regions, etc.
-- CREATE TABLE players ( ... );
-- CREATE TABLE map_regions ( ... );
-- CREATE TABLE entities ( ... );

