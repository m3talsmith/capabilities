-- Add down migration script here
DROP INDEX IF EXISTS idx_teams_users_team_id;

DROP INDEX IF EXISTS idx_teams_users_user_id;

DROP INDEX IF EXISTS idx_teams_users_team_role;

DROP INDEX IF EXISTS idx_teams_users_team_id_user_id;

DROP TABLE IF EXISTS teams_users;