-- Add up migration script here
CREATE TABLE
    users (
        id VARCHAR(255) PRIMARY KEY,
        first_name VARCHAR(255) NOT NULL,
        last_name VARCHAR(255) NOT NULL,
        username VARCHAR(255) NOT NULL UNIQUE,
        password_hash VARCHAR(255) NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        archived_at TIMESTAMP
    );

CREATE INDEX idx_users_id ON users (id);

CREATE INDEX idx_users_created_at ON users (created_at);

CREATE INDEX idx_users_updated_at ON users (updated_at);

CREATE INDEX idx_users_archived_at ON users (archived_at);

CREATE INDEX idx_users_username ON users (username);

CREATE INDEX idx_users_first_name ON users (first_name);

CREATE INDEX idx_users_last_name ON users (last_name);

CREATE TABLE
    authentications (
        id VARCHAR(255) PRIMARY KEY,
        user_id VARCHAR(255) NOT NULL,
        token VARCHAR(255) NOT NULL,
        expires_at TIMESTAMP NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        archived_at TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users (id)
    );

CREATE INDEX idx_authentications_id ON authentications (id);

CREATE INDEX idx_authentications_created_at ON authentications (created_at);

CREATE INDEX idx_authentications_updated_at ON authentications (updated_at);

CREATE INDEX idx_authentications_archived_at ON authentications (archived_at);

CREATE INDEX idx_authentications_user_id ON authentications (user_id);

CREATE INDEX idx_authentications_expires_at ON authentications (expires_at);

CREATE INDEX idx_authentications_token ON authentications (token);