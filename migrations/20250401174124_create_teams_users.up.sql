-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS teams_users (
        id VARCHAR(255) PRIMARY KEY,
        team_id VARCHAR(255) NOT NULL REFERENCES teams (id),
        user_id VARCHAR(255) NOT NULL REFERENCES users (id),
        team_role VARCHAR(255) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        WITH
            TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
            archived_at TIMESTAMP
        WITH
            TIME ZONE
    );

CREATE INDEX IF NOT EXISTS idx_teams_users_team_id ON teams_users (team_id);

CREATE INDEX IF NOT EXISTS idx_teams_users_team_role ON teams_users (team_role);

CREATE INDEX IF NOT EXISTS idx_teams_users_user_id ON teams_users (user_id);

CREATE UNIQUE INDEX IF NOT EXISTS idx_teams_users_team_id_user_id ON teams_users (team_id, user_id);

CREATE INDEX IF NOT EXISTS idx_teams_users_created_at ON teams_users (created_at);

CREATE INDEX IF NOT EXISTS idx_teams_users_updated_at ON teams_users (updated_at);

CREATE INDEX IF NOT EXISTS idx_teams_users_archived_at ON teams_users (archived_at);