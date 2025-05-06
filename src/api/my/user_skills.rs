use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::user::{User, UserError};
use crate::models::user_skill::{UserSkill, UserSkillError};
use crate::{
    delete_resource_where_fields, find_all_resources_where_fields,
    find_one_unarchived_resource_where_fields, insert_resource, update_resource,
};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    Authentication(AuthenticationError),
    User(UserError),
    UserSkill(UserSkillError),
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

impl From<UserSkillError> for ResponseError {
    fn from(error: UserSkillError) -> Self {
        ResponseError::UserSkill(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSkillsResponse {
    pub error: Option<ResponseError>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl UserSkillsResponse {
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
pub async fn get_user_skills(token: RawToken) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(UserSkillsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let find_user_params = vec![("id", &user_id)];
    let user = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(UserSkillsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(user.id.unwrap());
    let params = vec![("user_id", user_id)];
    match find_all_resources_where_fields!(UserSkill, params).await {
        Ok(user_skills) => status::Custom(
            Status::Ok,
            serde_json::to_value(UserSkillsResponse::success(
                serde_json::to_value(user_skills).unwrap(),
                Some("User skills fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error finding user skills: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(UserSkillsResponse::error(
                    UserSkillError::UserSkillsNotFound,
                    UserSkillError::UserSkillsNotFound.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserSkillRequest {
    pub skill_name: String,
    pub skill_level: i32,
}

#[post("/", data = "<user_skill>")]
pub async fn create_user_skill(
    token: RawToken,
    user_skill: Json<CreateUserSkillRequest>,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(UserSkillsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let find_user_params = vec![("id", &user_id)];
    let user = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(UserSkillsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(user.id.unwrap());
    let skill_name = DatabaseValue::String(user_skill.skill_name.clone().to_lowercase());
    let skill_level = DatabaseValue::Int(user_skill.skill_level.to_string());

    let params = vec![
        ("user_id", user_id),
        ("skill_name", skill_name),
        ("skill_level", skill_level),
    ];

    match insert_resource!(UserSkill, params).await {
        Ok(user_skill) => status::Custom(
            Status::Ok,
            serde_json::to_value(UserSkillsResponse::success(
                serde_json::to_value(user_skill).unwrap(),
                None,
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error creating user skill: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(UserSkillsResponse::error(
                    UserSkillError::UserSkillCreationFailed,
                    UserSkillError::UserSkillCreationFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserSkillRequest {
    pub skill_name: String,
    pub skill_level: i32,
}

#[put("/<user_skill_id>", data = "<user_skill>")]
pub async fn update_user_skill(
    token: RawToken,
    user_skill_id: &str,
    user_skill: Json<UpdateUserSkillRequest>,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(UserSkillsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let find_user_params = vec![("id", &user_id)];
    let user = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(UserSkillsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(user.id.unwrap());
    let skill_id = DatabaseValue::String(user_skill_id.to_string());
    let skill_name = DatabaseValue::String(user_skill.skill_name.clone().to_lowercase());
    let skill_level = DatabaseValue::Int(user_skill.skill_level.to_string());

    let params = vec![
        ("id", skill_id),
        ("user_id", user_id),
        ("skill_name", skill_name),
        ("skill_level", skill_level),
    ];

    match update_resource!(UserSkill, user_skill_id, params).await {
        Ok(user_skill) => status::Custom(
            Status::Ok,
            serde_json::to_value(UserSkillsResponse::success(
                serde_json::to_value(user_skill).unwrap(),
                Some("User skill updated successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error updating user skill: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(UserSkillsResponse::error(
                    UserSkillError::UserSkillUpdateFailed,
                    UserSkillError::UserSkillUpdateFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}

#[delete("/<user_skill_id>")]
pub async fn delete_user_skill(token: RawToken, user_skill_id: String) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(UserSkillsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let find_user_params = vec![("id", &user_id)];
    let user = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(UserSkillsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = user.id.unwrap();
    let params = vec![("id", user_skill_id), ("user_id", user_id)];
    match delete_resource_where_fields!(UserSkill, params).await {
        Ok(_) => status::Custom(
            Status::Ok,
            serde_json::to_value(UserSkillsResponse::success(
                serde_json::to_value(serde_json::json!(null)).unwrap(),
                Some("User skill deleted successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error deleting user skill: {:?}", err);
            status::Custom(
                Status::InternalServerError,
                serde_json::to_value(UserSkillsResponse::error(
                    UserSkillError::UserSkillDeletionFailed,
                    UserSkillError::UserSkillDeletionFailed.to_string(),
                ))
                .unwrap(),
            )
        }
    }
}
