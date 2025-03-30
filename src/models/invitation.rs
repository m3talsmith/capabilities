use crate::database::traits::DatabaseResource;
use crate::models::team_role::TeamRole;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub enum InvitationError {
    InvitationNotFound,
    InvitationCreationFailed,
    InvitationUpdateFailed,
    InvitationDeletionFailed,
}

impl std::fmt::Display for InvitationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvitationError::InvitationNotFound => write!(f, "Invitation not found"),
            InvitationError::InvitationCreationFailed => write!(f, "Invitation creation failed"),
            InvitationError::InvitationUpdateFailed => write!(f, "Invitation update failed"),
            InvitationError::InvitationDeletionFailed => write!(f, "Invitation deletion failed"),
        }
    }
}

impl std::error::Error for InvitationError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invitation {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub team_id: Option<String>,
    pub team_role: Option<TeamRole>,

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
    pub accepted_at: Option<OffsetDateTime>,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub rejected_at: Option<OffsetDateTime>,
}

impl Invitation {
    pub fn is_accepted(&self) -> bool {
        self.accepted_at.is_some()
    }

    pub fn is_rejected(&self) -> bool {
        self.rejected_at.is_some()
    }
}

impl DatabaseResource for Invitation {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Invitation {
            id: row.get("id"),
            user_id: row.get("user_id"),
            team_id: row.get("team_id"),
            team_role: row.get("team_role"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            accepted_at: row.get("accepted_at"),
            rejected_at: row.get("rejected_at"),
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
