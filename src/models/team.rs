use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub enum TeamError {
    TeamNotFound,
    TeamCreationFailed,
    TeamUpdateFailed,
    TeamDeletionFailed,
}

impl std::fmt::Display for TeamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamError::TeamNotFound => write!(f, "Team not found"),
            TeamError::TeamCreationFailed => write!(f, "Team creation failed"),
            TeamError::TeamUpdateFailed => write!(f, "Team update failed"),
            TeamError::TeamDeletionFailed => write!(f, "Team deletion failed"),
        }
    }
}

impl std::error::Error for TeamError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: Option<String>,
    pub owner_id: Option<String>,
    pub team_name: Option<String>,
    pub team_description: Option<String>,

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

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub archived_at: Option<OffsetDateTime>,
}

impl DatabaseResource for Team {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Team {
            id: row.get("id"),
            owner_id: row.get("owner_id"),
            team_name: row.get("team_name"),
            team_description: row.get("team_description"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            archived_at: row.get("archived_at"),
        })
    }

    fn has_id() -> bool {
        true
    }

    fn is_archivable() -> bool {
        true
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
