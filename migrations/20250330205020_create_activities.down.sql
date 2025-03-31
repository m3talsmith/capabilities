-- Add down migration script here
DROP INDEX IF EXISTS idx_activities_activity_name;
DROP INDEX IF EXISTS idx_activities_activity_description;
DROP INDEX IF EXISTS idx_activities_assigned_to;
DROP INDEX IF EXISTS idx_activities_team_id;
DROP INDEX IF EXISTS idx_activities_duration_in_hours;
DROP INDEX IF EXISTS idx_activities_created_at;
DROP INDEX IF EXISTS idx_activities_updated_at;
DROP INDEX IF EXISTS idx_activities_archived_at;
DROP INDEX IF EXISTS idx_activities_started_at;
DROP INDEX IF EXISTS idx_activities_paused_at;
DROP INDEX IF EXISTS idx_activities_ended_at;
DROP INDEX IF EXISTS idx_activities_id;

DROP TABLE IF EXISTS activities;