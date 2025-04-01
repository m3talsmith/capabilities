use crate::database::traits::DatabaseResource;
use rocket::serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TeamUserError {
    TeamUserNotFound,
    TeamUserCreationFailed,
    TeamUserDeletionFailed,
}

impl std::fmt::Display for TeamUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamUserError::TeamUserNotFound => write!(f, "Team user not found"),
            TeamUserError::TeamUserCreationFailed => write!(f, "Team user creation failed"),
            TeamUserError::TeamUserDeletionFailed => write!(f, "Team user deletion failed"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamUser {
    pub team_id: Option<String>,
    pub user_id: Option<String>,
}

impl DatabaseResource for TeamUser {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(TeamUser {
            team_id: row.get("team_id"),
            user_id: row.get("user_id"),
        })
    }

    fn has_id() -> bool {
        false
    }

    fn is_archivable() -> bool {
        false
    }

    fn is_updatable() -> bool {
        false
    }

    fn is_creatable() -> bool {
        false
    }

    fn is_expirable() -> bool {
        false
    }
}
