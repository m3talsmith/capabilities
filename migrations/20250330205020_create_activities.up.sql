-- Add up migration script here
CREATE TABLE IF NOT EXISTS activities (
    id VARCHAR(255) PRIMARY KEY,
    activity_name VARCHAR(255) NOT NULL,
    activity_description VARCHAR(255) NOT NULL,
    assigned_to VARCHAR(255) NOT NULL REFERENCES users(id),
    team_id VARCHAR(255) NOT NULL REFERENCES teams(id),
    duration_in_hours BIGINT NOT NULL DEFAULT 1,
    started_at TIMESTAMP WITH TIME ZONE NOT NULL,
    paused_at TIMESTAMP WITH TIME ZONE,
    ended_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    archived_at TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_activities_id ON activities (id);
CREATE INDEX IF NOT EXISTS idx_activities_activity_name ON activities (activity_name);
CREATE INDEX IF NOT EXISTS idx_activities_activity_description ON activities (activity_description);
CREATE INDEX IF NOT EXISTS idx_activities_assigned_to ON activities (assigned_to);
CREATE INDEX IF NOT EXISTS idx_activities_team_id ON activities (team_id);
CREATE INDEX IF NOT EXISTS idx_activities_duration_in_hours ON activities (duration_in_hours);
CREATE INDEX IF NOT EXISTS idx_activities_created_at ON activities (created_at);
CREATE INDEX IF NOT EXISTS idx_activities_updated_at ON activities (updated_at);
CREATE INDEX IF NOT EXISTS idx_activities_archived_at ON activities (archived_at);
CREATE INDEX IF NOT EXISTS idx_activities_started_at ON activities (started_at);
CREATE INDEX IF NOT EXISTS idx_activities_paused_at ON activities (paused_at);
CREATE INDEX IF NOT EXISTS idx_activities_ended_at ON activities (ended_at);
