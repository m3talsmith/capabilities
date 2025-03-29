use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use rocket::serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserSkillError {
    UserSkillsNotFound,
    UserSkillCreationFailed,
    UserSkillNotFound,
    UserSkillUpdateFailed,
    UserSkillDeletionFailed,
}

impl std::fmt::Display for UserSkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserSkillError::UserSkillsNotFound => write!(f, "User skills not found"),
            UserSkillError::UserSkillCreationFailed => write!(f, "User skill creation failed"),
            UserSkillError::UserSkillNotFound => write!(f, "User skill not found"),
            UserSkillError::UserSkillUpdateFailed => write!(f, "User skill update failed"),
            UserSkillError::UserSkillDeletionFailed => write!(f, "User skill deletion failed"),
        }
    }
}

impl std::error::Error for UserSkillError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSkill {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub skill_name: Option<String>,
    pub skill_level: Option<i32>,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub created_at: Option<OffsetDateTime>,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub updated_at: Option<OffsetDateTime>,
}

impl DatabaseResource for UserSkill {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(UserSkill {
            id: row.get("id"),
            user_id: row.get("user_id"),
            skill_name: row.get("skill_name"),
            skill_level: row.get("skill_level"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    fn has_id() -> bool {
        true
    }

    fn is_archivable() -> bool {
        false
    }

    fn is_updatable() -> bool {
        true
    }

    fn is_creatable() -> bool {
        true
    }

    fn is_expirable() -> bool {
        false
    }
}
