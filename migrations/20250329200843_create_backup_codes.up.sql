-- Add up migration script here
CREATE TABLE IF NOT EXISTS backup_codes (
    id VARCHAR(255) PRIMARY KEY UNIQUE,
    code VARCHAR(255) NOT NULL UNIQUE,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_backup_codes_id ON backup_codes (id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_backup_codes_code ON backup_codes (code);
CREATE INDEX IF NOT EXISTS idx_backup_codes_user_id ON backup_codes (user_id);
CREATE INDEX IF NOT EXISTS idx_backup_codes_created_at ON backup_codes (created_at);
CREATE INDEX IF NOT EXISTS idx_backup_codes_updated_at ON backup_codes (updated_at);
CREATE INDEX IF NOT EXISTS idx_backup_codes_archived_at ON backup_codes (archived_at);
