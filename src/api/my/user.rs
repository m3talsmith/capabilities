use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::find_one_unarchived_resource_where_fields;
use crate::models::authentication::AuthenticationError;
use crate::models::user::{User, UserError};
use crate::update_resource;
use crate::utils::passwords::hash_password;
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    Authentication(AuthenticationError),
    User(UserError),
}

impl From<AuthenticationError> for ResponseError {
    fn from(error: AuthenticationError) -> Self {
        ResponseError::Authentication(error)
    }
}

impl From<UserError> for ResponseError {
    fn from(error: UserError) -> Self {
        ResponseError::User(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub error: Option<ResponseError>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl UserResponse {
    pub fn success(data: Value, message: Option<String>) -> Self {
        Self {
            error: None,
            message,
            data: Some(data),
        }
    }

    pub fn error(error: impl Into<ResponseError>, message: String) -> Self {
        Self {
            error: Some(error.into()),
            message: Some(message),
            data: None,
        }
    }
}

#[get("/")]
pub async fn get_user(token: RawToken) -> rocket::response::status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(UserResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let id_param = ("id", &user_id);
    let find_user_params = vec![id_param];
    match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => status::Custom(
            Status::Ok,
            serde_json::json!(UserResponse::success(
                serde_json::to_value(user).unwrap(),
                Some("User fetched successfully".to_string())
            )),
        ),
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(UserResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[put("/", data = "<user>")]
pub async fn update_user(
    token: RawToken,
    user: Json<User>,
) -> rocket::response::status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(UserResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let id_param = ("id", &user_id);
    let find_user_params = vec![id_param];
    let _ = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(UserResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_params = vec![
        (
            "first_name",
            DatabaseValue::String(user.0.first_name.unwrap_or_default()),
        ),
        (
            "last_name",
            DatabaseValue::String(user.0.last_name.unwrap_or_default()),
        ),
        (
            "username",
            DatabaseValue::String(user.0.username.unwrap_or_default()),
        ),
    ];

    let user_id = token_value.user_id.clone();
    match update_resource!(User, user_id, user_params).await {
        Ok(user) => status::Custom(
            Status::Ok,
            serde_json::json!(UserResponse::success(
                serde_json::to_value(user).unwrap(),
                Some("User updated successfully".to_string())
            )),
        ),
        Err(err) => {
            println!("Error updating user: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::json!(UserResponse::error(
                    UserError::UserUpdateFailed,
                    UserError::UserUpdateFailed.to_string()
                )),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[post("/change-password", data = "<user_change_password_request>")]
pub async fn change_password(
    token: RawToken,
    user_change_password_request: Json<UserChangePasswordRequest>,
) -> rocket::response::status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(UserResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let hashed_original_password = hash_password(&user_change_password_request.old_password);

    let user_id_param = ("id", &user_id);
    let password_param = (
        "password_hash",
        &DatabaseValue::String(hashed_original_password),
    );
    let find_user_params = vec![user_id_param, password_param];
    let _ = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(UserResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let hashed_new_password = hash_password(&user_change_password_request.new_password);

    let user_params = vec![("password_hash", DatabaseValue::String(hashed_new_password))];
    match update_resource!(User, user_id, user_params).await {
        Ok(user) => status::Custom(
            Status::Ok,
            serde_json::to_value(UserResponse::success(
                serde_json::to_value(user).unwrap(),
                Some("Password updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error updating user: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::json!(UserResponse::error(
                    UserError::UserUpdateFailed,
                    UserError::UserUpdateFailed.to_string()
                )),
            );
        }
    }
}
