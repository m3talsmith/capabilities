-- Add up migration script here
CREATE TABLE IF NOT EXISTS teams (
    id VARCHAR(255) PRIMARY KEY,
    owner_id VARCHAR(255) NOT NULL REFERENCES users(id),
    team_name VARCHAR(255) NOT NULL,
    team_description VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_teams_id ON teams (id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_teams_owner_id ON teams (owner_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_teams_team_name ON teams (team_name);
CREATE INDEX IF NOT EXISTS idx_teams_created_at ON teams (created_at);
CREATE INDEX IF NOT EXISTS idx_teams_updated_at ON teams (updated_at);
CREATE INDEX IF NOT EXISTS idx_teams_archived_at ON teams (archived_at);

CREATE TYPE team_role_enum AS ENUM ('admin', 'manager', 'member');

CREATE TABLE IF NOT EXISTS invitations (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL REFERENCES users(id),
    team_id VARCHAR(255) NOT NULL REFERENCES teams(id),
    team_role team_role_enum NOT NULL DEFAULT 'member',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    accepted_at TIMESTAMP WITH TIME ZONE,
    rejected_at TIMESTAMP WITH TIME ZONE,
    archived_at TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_invitations_id ON invitations (id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_invitations_user_id ON invitations (user_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_invitations_team_id ON invitations (team_id);
CREATE INDEX IF NOT EXISTS idx_invitations_created_at ON invitations (created_at);
CREATE INDEX IF NOT EXISTS idx_invitations_updated_at ON invitations (updated_at);
CREATE INDEX IF NOT EXISTS idx_invitations_accepted_at ON invitations (accepted_at);
CREATE INDEX IF NOT EXISTS idx_invitations_rejected_at ON invitations (rejected_at);
CREATE INDEX IF NOT EXISTS idx_invitations_archived_at ON invitations (archived_at);