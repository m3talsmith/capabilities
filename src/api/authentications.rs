use crate::api::token::{RawToken, VerifiedToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::{Authentication, AuthenticationError};
use crate::models::user::{User, UserError};
use crate::{
    delete_resource_where_fields, find_one_archived_resource_where_fields,
    find_one_resource_where_fields, find_one_unarchived_resource_where_fields, insert_resource,
    update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    Authentication(AuthenticationError),
    User(UserError),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub error: Option<ResponseError>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl AuthenticationResponse {
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
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
}

#[post("/", data = "<authentication_request>")]
pub async fn login(authentication_request: Json<AuthenticationRequest>) -> status::Custom<Value> {
    let hashed_password = format!(
        "{:x}",
        Sha256::digest(authentication_request.password.as_bytes())
    );

    let username = DatabaseValue::String(authentication_request.username.clone());
    let password = DatabaseValue::String(hashed_password);

    let login_params = vec![("username", &username), ("password_hash", &password)];
    let user = match find_one_unarchived_resource_where_fields!(User, login_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(AuthenticationResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = user.id.unwrap();

    let auth_params = vec![("user_id", &user_id)];
    match find_one_unarchived_resource_where_fields!(Authentication, auth_params).await {
        Ok(authentication) => {
            let id = DatabaseValue::String(authentication.id.clone());
            match update_resource!(Authentication, id, vec![]).await {
                Ok(_) => status::Custom(
                    Status::Ok,
                    serde_json::to_value(AuthenticationResponse::success(
                        serde_json::to_value(authentication).unwrap(),
                        None,
                    ))
                    .unwrap(),
                ),
                Err(err) => {
                    println!("Error updating authentication: {:?}", err);
                    return status::Custom(
                        Status::InternalServerError,
                        serde_json::to_value(AuthenticationResponse::error(
                            AuthenticationError::SessionUpdateFailed,
                            AuthenticationError::SessionUpdateFailed.to_string(),
                        ))
                        .unwrap(),
                    );
                }
            }
        }
        Err(_) => {
            let token = Uuid::new_v4().to_string();
            match insert_resource!(
                Authentication,
                vec![
                    ("user_id", DatabaseValue::String(user_id)),
                    ("token", DatabaseValue::String(token)),
                ]
            )
            .await
            {
                Ok(authentication) => status::Custom(
                    Status::Ok,
                    serde_json::to_value(AuthenticationResponse::success(
                        serde_json::to_value(authentication).unwrap(),
                        None,
                    ))
                    .unwrap(),
                ),
                Err(err) => {
                    println!("Error creating authentication: {:?}", err);
                    return status::Custom(
                        Status::InternalServerError,
                        serde_json::to_value(AuthenticationResponse::error(
                            AuthenticationError::SessionCreationFailed,
                            AuthenticationError::SessionCreationFailed.to_string(),
                        ))
                        .unwrap(),
                    );
                }
            }
        }
    }
}

#[delete("/")]
pub async fn logout(token: RawToken) -> status::Custom<Value> {
    if token.value.is_empty() {
        return status::Custom(
            Status::BadRequest,
            serde_json::to_value(AuthenticationResponse::error(
                AuthenticationError::SessionNotFound,
                AuthenticationError::SessionNotFound.to_string(),
            ))
            .unwrap(),
        );
    }
    let token_value = match VerifiedToken::from_raw(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error verifying token: {:?}", err);
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(AuthenticationResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };
    let token_str = token_value.raw_token.unwrap().clone();
    let logout_params = vec![("token", &token_str)];
    match delete_resource_where_fields!(Authentication, logout_params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(AuthenticationResponse::success(
                serde_json::json!(null),
                Some("Logged out successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error deleting authentication: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(AuthenticationResponse::error(
                    AuthenticationError::SessionDeletionFailed,
                    AuthenticationError::SessionDeletionFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[post("/register", data = "<register_request>")]
pub async fn register(register_request: Json<RegisterRequest>) -> status::Custom<Value> {
    let hashed_password = format!("{:x}", Sha256::digest(register_request.password.as_bytes()));

    let first_name = DatabaseValue::String(register_request.first_name.clone());
    let last_name = DatabaseValue::String(register_request.last_name.clone());
    let username = DatabaseValue::String(register_request.username.clone());
    let email = DatabaseValue::String(register_request.email.clone());
    let password = DatabaseValue::String(hashed_password);

    let user_params = vec![
        ("username", &username),
        ("password_hash", &password),
        ("email", &email),
    ];
    let _ = match find_one_archived_resource_where_fields!(User, user_params).await {
        Ok(user) => {
            let id = user.id.clone().unwrap();
            let id_value = DatabaseValue::String(id);
            let archived_at = DatabaseValue::None;

            match update_resource!(User, id_value, vec![("archived_at", archived_at)]).await {
                Ok(user) => {
                    return status::Custom(
                        Status::Ok,
                        serde_json::to_value(AuthenticationResponse::success(
                            serde_json::to_value(user).unwrap(),
                            None,
                        ))
                        .unwrap(),
                    )
                }
                Err(err) => {
                    println!("Error updating user: {:?}", err);
                    return status::Custom(
                        Status::InternalServerError,
                        serde_json::to_value(AuthenticationResponse::error(
                            UserError::UserUpdateFailed,
                            UserError::UserUpdateFailed.to_string(),
                        ))
                        .unwrap(),
                    );
                }
            }
        }
        Err(_) => (),
    };

    let register_params = vec![
        ("first_name", first_name),
        ("last_name", last_name),
        ("username", username),
        ("password_hash", password),
        ("email", email),
    ];
    match insert_resource!(User, register_params).await {
        Ok(user) => status::Custom(
            Status::Ok,
            serde_json::to_value(AuthenticationResponse::success(
                serde_json::to_value(user).unwrap(),
                None,
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error registering user: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(AuthenticationResponse::error(
                    AuthenticationError::RegistrationFailed,
                    AuthenticationError::RegistrationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[delete("/register")]
pub async fn unregister(token: RawToken) -> status::Custom<Value> {
    let token_value = match VerifiedToken::from_raw(token).await {
        Ok(token) => token,
        Err(_) => {
            return status::Custom(
                Status::BadRequest,
                serde_json::to_value(AuthenticationResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };
    let user_id = token_value.user_id.clone();

    let user_params = vec![("id", &user_id)];
    let _ = match find_one_resource_where_fields!(User, user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(AuthenticationResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let auth_params = vec![("user_id", &user_id)];
    let _ = match delete_resource_where_fields!(Authentication, auth_params).await {
        Ok(_) => (),
        Err(err) => {
            println!("Error deleting authentication: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(AuthenticationResponse::error(
                    AuthenticationError::SessionNotFound,
                    AuthenticationError::SessionNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let delete_params = vec![("id", &user_id)];
    match delete_resource_where_fields!(User, delete_params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(AuthenticationResponse::success(
                serde_json::json!(null),
                Some("User deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error deleting user: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(AuthenticationResponse::error(
                    UserError::UserDeletionFailed,
                    UserError::UserDeletionFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}
