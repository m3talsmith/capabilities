use crate::database::traits::DatabaseResource;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use rocket::serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Error, Row};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,

    #[allow(dead_code)]
    #[serde(skip_serializing, skip_deserializing)]
    pub password: Option<String>,

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

impl DatabaseResource for User {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(User {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            username: row.get("username"),
            password: row.get("user_password"),
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
}
