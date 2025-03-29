use crate::api::token::{RawToken, VerifiedToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::backup_code::{BackupCode, BackupCodeError};
use crate::models::user::{User, UserError};
use crate::utils::backup_codes::generate_backup_codes;
use crate::{
    delete_resource_where_fields, find_all_unarchived_resources_where_fields,
    find_one_unarchived_resource_where_fields, insert_resource,
};
use rocket::http::Status;
use rocket::response::status;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    Authentication(AuthenticationError),
    User(UserError),
    BackupCode(BackupCodeError),
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

impl From<BackupCodeError> for ResponseError {
    fn from(error: BackupCodeError) -> Self {
        ResponseError::BackupCode(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupCodesResponse {
    pub data: Option<Value>,
    pub message: Option<String>,
    pub error: Option<ResponseError>,
}

impl BackupCodesResponse {
    pub fn success(data: Value, message: Option<String>) -> Self {
        Self {
            data: Some(data),
            message,
            error: None,
        }
    }

    pub fn error(error: impl Into<ResponseError>, message: String) -> Self {
        Self {
            data: None,
            message: Some(message),
            error: Some(error.into()),
        }
    }
}

#[get("/")]
pub async fn get_backup_codes(token: RawToken) -> status::Custom<Value> {
    if token.value.is_empty() {
        return status::Custom(
            Status::BadRequest,
            serde_json::to_value(BackupCodesResponse::error(
                AuthenticationError::InvalidToken,
                AuthenticationError::InvalidToken.to_string(),
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
                serde_json::to_value(BackupCodesResponse::error(
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
            println!("Error fetching backup codes: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(BackupCodesResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let codes_params = vec![("user_id", &user_id)];
    let codes_result = find_all_unarchived_resources_where_fields!(BackupCode, codes_params).await;
    match codes_result {
        Ok(codes) => status::Custom(
            Status::Ok,
            serde_json::to_value(BackupCodesResponse::success(
                serde_json::to_value(
                    codes
                        .into_iter()
                        .map(|code| code.code.unwrap())
                        .collect::<Vec<String>>(),
                )
                .unwrap(),
                Some("Backup codes fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching backup codes: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(BackupCodesResponse::error(
                    BackupCodeError::CodeNotFound,
                    BackupCodeError::CodeNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/generate")]
pub async fn regenerate_backup_codes(token: RawToken) -> status::Custom<Value> {
    if token.value.is_empty() {
        return status::Custom(
            Status::BadRequest,
            serde_json::to_value(BackupCodesResponse::error(
                AuthenticationError::InvalidToken,
                AuthenticationError::InvalidToken.to_string(),
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
                serde_json::to_value(BackupCodesResponse::error(
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
            println!("Error fetching user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(BackupCodesResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let old_codes_params = vec![("user_id", &user_id)];
    let old_codes =
        match find_all_unarchived_resources_where_fields!(BackupCode, old_codes_params).await {
            Ok(codes) => codes,
            Err(err) => {
                println!("Error fetching backup codes: {:?}", err);
                return status::Custom(
                    Status::NotFound,
                    serde_json::to_value(BackupCodesResponse::error(
                        BackupCodeError::CodeNotFound,
                        BackupCodeError::CodeNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };
    let old_codes_ids = old_codes
        .iter()
        .map(|code| code.id.clone().unwrap())
        .collect::<Vec<String>>();
    for id in old_codes_ids {
        let id_param = ("id", &id);
        let delete_params = vec![id_param];
        let _ = match delete_resource_where_fields!(BackupCode, delete_params).await {
            Ok(_) => (),
            Err(err) => {
                println!("Error deleting backup code: {:?}", err);
                return status::Custom(
                    Status::InternalServerError,
                    serde_json::to_value(BackupCodesResponse::error(
                        BackupCodeError::CodeDeletionFailed,
                        BackupCodeError::CodeDeletionFailed.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };
    }

    let codes = generate_backup_codes().await;
    let user_id_value = user_id.clone();
    for code in codes {
        let backup_code_params = vec![
            ("code", DatabaseValue::String(code.clone())),
            ("user_id", user_id_value.clone()),
        ];
        let _ = match insert_resource!(BackupCode, backup_code_params).await {
            Ok(_) => (),
            Err(err) => {
                println!("Error inserting backup code: {:?}", err);
                return status::Custom(
                    Status::InternalServerError,
                    serde_json::to_value(BackupCodesResponse::error(
                        BackupCodeError::CodeCreationFailed,
                        BackupCodeError::CodeCreationFailed.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };
    }

    let codes_params = vec![("user_id", &user_id)];
    let codes_result = find_all_unarchived_resources_where_fields!(BackupCode, codes_params).await;
    match codes_result {
        Ok(codes) => status::Custom(
            Status::Ok,
            serde_json::to_value(BackupCodesResponse::success(
                serde_json::to_value(
                    codes
                        .into_iter()
                        .map(|code| code.code.unwrap())
                        .collect::<Vec<String>>(),
                )
                .unwrap(),
                Some("Backup codes generated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching backup codes: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(BackupCodesResponse::error(
                    BackupCodeError::CodeNotFound,
                    BackupCodeError::CodeNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}
