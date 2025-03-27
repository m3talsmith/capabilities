-- Add down migration script here
DROP INDEX idx_authentications_token;

DROP INDEX idx_authentications_expires_at;

DROP INDEX idx_authentications_user_id;

DROP INDEX idx_authentications_archived_at;

DROP INDEX idx_authentications_updated_at;

DROP INDEX idx_authentications_created_at;

DROP INDEX idx_authentications_id;

DROP TABLE authentications;

DROP INDEX idx_users_last_name;

DROP INDEX idx_users_first_name;

DROP INDEX idx_users_username;

DROP INDEX idx_users_archived_at;

DROP INDEX idx_users_updated_at;

DROP INDEX idx_users_created_at;

DROP INDEX idx_users_id;

DROP TABLE users;