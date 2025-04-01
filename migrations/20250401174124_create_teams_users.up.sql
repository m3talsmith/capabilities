-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS teams_users (
        team_id TEXT NOT NULL,
        user_id TEXT NOT NULL,
        PRIMARY KEY (team_id, user_id),
        FOREIGN KEY (team_id) REFERENCES teams (id),
        FOREIGN KEY (user_id) REFERENCES users (id)
    );

CREATE INDEX IF NOT EXISTS idx_teams_users_team_id ON teams_users (team_id);

CREATE INDEX IF NOT EXISTS idx_teams_users_user_id ON teams_users (user_id);