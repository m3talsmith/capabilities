use crate::models::authentication::Authentication;
use crate::utils::time::{deserialize_offset_date_time, serialize_offset_date_time};
use crate::{find_one_resource_where_fields, models::authentication::AuthenticationError};
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub user_id: String,
    pub token: String,
    pub expires_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct VerifiedToken {
    pub raw_token: Option<String>,
    #[serde(rename = "ssoToken")]
    pub user_id: String,

    #[serde(
        serialize_with = "serialize_offset_date_time",
        deserialize_with = "deserialize_offset_date_time"
    )]
    pub expires_at: Option<OffsetDateTime>,
}

impl VerifiedToken {
    pub fn new(raw_token: String, user_id: String, expires_at: Option<OffsetDateTime>) -> Self {
        Self {
            raw_token: Some(raw_token),
            user_id,
            expires_at,
        }
    }

    pub async fn from_raw(raw_token: RawToken) -> Result<Self, AuthenticationError> {
        let params = vec![("token", &raw_token.value)];
        let authentication = match find_one_resource_where_fields!(Authentication, params).await {
            Ok(authentication) => authentication,
            Err(_) => return Err(AuthenticationError::InvalidToken),
        };
        if authentication.expires_at.is_none()
            || authentication.expires_at.as_ref().unwrap().to_string()
                < OffsetDateTime::now_utc().format(&Rfc3339).unwrap()
        {
            return Err(AuthenticationError::TokenExpired);
        }
        Ok(Self::new(
            raw_token.value,
            authentication.user_id,
            Some(authentication.expires_at.unwrap().clone()),
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RawToken {
    pub value: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RawToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let token = request
            .headers()
            .get_one("Authorization")
            .map(|header| header.split(" ").nth(1).unwrap_or(""));
        Outcome::Success(
            request
                .local_cache(|| RawToken {
                    value: token.unwrap_or("").to_string(),
                })
                .clone(),
        )
    }
}
