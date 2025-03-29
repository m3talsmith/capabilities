-- Add down migration script here
DROP INDEX IF EXISTS idx_backup_codes_id;
DROP INDEX IF EXISTS idx_backup_codes_code;
DROP INDEX IF EXISTS idx_backup_codes_user_id;
DROP INDEX IF EXISTS idx_backup_codes_created_at;
DROP INDEX IF EXISTS idx_backup_codes_updated_at;
DROP INDEX IF EXISTS idx_backup_codes_archived_at;
DROP TABLE IF EXISTS backup_codes;