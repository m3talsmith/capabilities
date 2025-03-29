-- Add down migration script here
DROP INDEX IF EXISTS idx_authentications_token;

DROP INDEX IF EXISTS idx_authentications_expires_at;

DROP INDEX IF EXISTS idx_authentications_user_id;

DROP INDEX IF EXISTS idx_authentications_archived_at;

DROP INDEX IF EXISTS idx_authentications_updated_at;

DROP INDEX IF EXISTS idx_authentications_created_at;

DROP INDEX IF EXISTS idx_authentications_id;

DROP TABLE IF EXISTS authentications;

DROP INDEX IF EXISTS idx_users_last_name;

DROP INDEX IF EXISTS idx_users_first_name;

DROP INDEX IF EXISTS idx_users_username;

DROP INDEX IF EXISTS idx_users_archived_at;

DROP INDEX IF EXISTS idx_users_updated_at;

DROP INDEX IF EXISTS idx_users_created_at;

DROP INDEX IF EXISTS idx_users_id;

DROP TABLE IF EXISTS users;