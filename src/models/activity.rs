use crate::{
    database::traits::DatabaseResource,
    utils::time::{deserialize_offset_date_time, serialize_offset_date_time},
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActivityError {
    ActivityNotFound,
    ActivityAlreadyExists,
    ActivityNotStarted,
    ActivityAlreadyStarted,
    ActivityAlreadyEnded,
    ActivityNotPaused,
    ActivityNotEnded,
    ActivityDeletionError,
    ActivityUpdateError,
    ActivityCreationError,
    ActivityCreationFailed,
    ActivityUpdateFailed,
    ActivityDeletionFailed,
}

impl std::fmt::Display for ActivityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityError::ActivityNotFound => write!(f, "Activity not found"),
            ActivityError::ActivityAlreadyExists => write!(f, "Activity already exists"),
            ActivityError::ActivityNotStarted => write!(f, "Activity not started"),
            ActivityError::ActivityAlreadyStarted => write!(f, "Activity already started"),
            ActivityError::ActivityAlreadyEnded => write!(f, "Activity already ended"),
            ActivityError::ActivityNotPaused => write!(f, "Activity not paused"),
            ActivityError::ActivityNotEnded => write!(f, "Activity not ended"),
            ActivityError::ActivityDeletionError => write!(f, "Activity deletion error"),
            ActivityError::ActivityUpdateError => write!(f, "Activity update error"),
            ActivityError::ActivityCreationError => write!(f, "Activity creation error"),
            ActivityError::ActivityCreationFailed => write!(f, "Failed to create activity"),
            ActivityError::ActivityUpdateFailed => write!(f, "Failed to update activity"),
            ActivityError::ActivityDeletionFailed => write!(f, "Failed to delete activity"),
        }
    }
}

impl std::error::Error for ActivityError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub id: Option<String>,
    pub activity_name: Option<String>,
    pub activity_description: Option<String>,
    pub assigned_to: Option<String>,
    pub team_id: Option<String>,
    pub duration_in_hours: Option<i64>,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub started_at: Option<OffsetDateTime>,
    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub paused_at: Option<OffsetDateTime>,
    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub ended_at: Option<OffsetDateTime>,
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

impl DatabaseResource for Activity {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Activity {
            id: row.get("id"),
            activity_name: row.get("activity_name"),
            activity_description: row.get("activity_description"),
            assigned_to: row.get("assigned_to"),
            team_id: row.get("team_id"),
            duration_in_hours: row.get("duration_in_hours"),
            started_at: row.get("started_at"),
            paused_at: row.get("paused_at"),
            ended_at: row.get("ended_at"),
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
