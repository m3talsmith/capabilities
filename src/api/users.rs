use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::find_all_unarchived_resources_where_fields;
use crate::models::authentication::AuthenticationError;
use crate::models::user::{User, UserError};
use rocket::http::Status;
use rocket::response::status;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    User(UserError),
    Authentication(AuthenticationError),
}

impl From<UserError> for ResponseError {
    fn from(error: UserError) -> Self {
        ResponseError::User(error)
    }
}

impl From<AuthenticationError> for ResponseError {
    fn from(error: AuthenticationError) -> Self {
        ResponseError::Authentication(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersResponse {
    pub message: Option<String>,
    pub data: Option<Value>,
    pub error: Option<ResponseError>,
}

impl UsersResponse {
    pub fn success(data: Value, message: Option<String>) -> Self {
        Self {
            message,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: impl Into<ResponseError>, message: String) -> Self {
        Self {
            message: Some(message),
            data: None,
            error: Some(error.into()),
        }
    }
}

#[get("/")]
pub async fn get_users(token: RawToken) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token_value) => token_value,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(UsersResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let users_params: Vec<(&str, &DatabaseValue)> = vec![];
    match find_all_unarchived_resources_where_fields!(User, users_params).await {
        Ok(users) => status::Custom(
            Status::Ok,
            serde_json::to_value(UsersResponse::success(
                serde_json::to_value(users).unwrap(),
                Some("Users fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching users: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(UsersResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}
