-- Add down migration script here
DROP INDEX IF EXISTS idx_user_skills_id;

DROP INDEX IF EXISTS idx_user_skills_user_id;

DROP INDEX IF EXISTS idx_user_skills_skill_name;

DROP INDEX IF EXISTS idx_user_skills_skill_level;

DROP INDEX IF EXISTS idx_user_skills_created_at;

DROP INDEX IF EXISTS idx_user_skills_updated_at;

DROP TABLE IF EXISTS user_skills;