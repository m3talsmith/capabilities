-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS user_skills (
        id VARCHAR(255) PRIMARY KEY,
        user_id VARCHAR(255) NOT NULL REFERENCES users (id),
        skill_name VARCHAR(255) NOT NULL,
        skill_level INTEGER NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
        UNIQUE (user_id, skill_name),
        FOREIGN KEY (user_id) REFERENCES users (id)
    );

CREATE INDEX IF NOT EXISTS idx_user_skills_id ON user_skills (id);

CREATE INDEX IF NOT EXISTS idx_user_skills_user_id ON user_skills (user_id);

CREATE INDEX IF NOT EXISTS idx_user_skills_skill_name ON user_skills (skill_name);

CREATE INDEX IF NOT EXISTS idx_user_skills_skill_level ON user_skills (skill_level);

CREATE INDEX IF NOT EXISTS idx_user_skills_created_at ON user_skills (created_at);

CREATE INDEX IF NOT EXISTS idx_user_skills_updated_at ON user_skills (updated_at);