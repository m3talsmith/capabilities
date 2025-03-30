-- Add down migration script here
DROP INDEX IF EXISTS idx_invitations_id;
DROP INDEX IF EXISTS idx_invitations_user_id;
DROP INDEX IF EXISTS idx_invitations_team_id;
DROP INDEX IF EXISTS idx_invitations_accepted_at;
DROP INDEX IF EXISTS idx_invitations_rejected_at;
DROP INDEX IF EXISTS idx_invitations_created_at;
DROP INDEX IF EXISTS idx_invitations_updated_at;
DROP INDEX IF EXISTS idx_invitations_archived_at;
DROP TABLE IF EXISTS invitations;

DROP TYPE IF EXISTS team_role_enum;

DROP INDEX IF EXISTS idx_teams_id;
DROP INDEX IF EXISTS idx_teams_owner_id;
DROP INDEX IF EXISTS idx_teams_team_name;
DROP INDEX IF EXISTS idx_teams_created_at;
DROP INDEX IF EXISTS idx_teams_updated_at;
DROP INDEX IF EXISTS idx_teams_archived_at;
DROP TABLE IF EXISTS teams;
