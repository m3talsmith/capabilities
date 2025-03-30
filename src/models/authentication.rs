use crate::database::traits::DatabaseResource;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error as SqlxError, Row};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuthenticationError {
    UserNotFound,
    InvalidCredentials,
    SessionCreationFailed,
    SessionDeletionFailed,
    SessionUpdateFailed,
    SessionNotFound,
    InvalidToken,
    TokenExpired,
    RegistrationFailed,
}

impl std::fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthenticationError::UserNotFound => write!(f, "User not found"),
            AuthenticationError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthenticationError::SessionCreationFailed => write!(f, "Failed to create session"),
            AuthenticationError::SessionDeletionFailed => write!(f, "Failed to delete session"),
            AuthenticationError::SessionUpdateFailed => write!(f, "Failed to update session"),
            AuthenticationError::SessionNotFound => write!(f, "Session not found"),
            AuthenticationError::InvalidToken => write!(f, "Invalid token"),
            AuthenticationError::TokenExpired => write!(f, "Token expired"),
            AuthenticationError::RegistrationFailed => write!(f, "Registration failed"),
        }
    }
}

impl std::error::Error for AuthenticationError {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authentication {
    pub id: String,
    pub user_id: String,
    pub token: String,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub expires_at: Option<OffsetDateTime>,

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

impl DatabaseResource for Authentication {
    fn from_row(row: &PgRow) -> Result<Self, SqlxError> {
        Ok(Authentication {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token: row.get("token"),
            expires_at: row.get("expires_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            archived_at: row.get("archived_at"),
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
        true
    }
}

fn serialize_offset_date_time<S>(
    dt: &Option<OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(dt) => serializer.serialize_str(&dt.format(&Rfc3339).unwrap()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_offset_date_time<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => OffsetDateTime::parse(&s, &Rfc3339)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
