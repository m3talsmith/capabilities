use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::find_all_unarchived_resources_where_fields;
use crate::find_one_unarchived_resource_where_fields;
use crate::models::activity::{Activity, ActivityError};
use crate::models::authentication::AuthenticationError;
use crate::models::team::TeamError;
use crate::update_resource;
use rocket::http::Status;
use rocket::response::status;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    Activity(ActivityError),
    Authentication(AuthenticationError),
    Team(TeamError),
}

impl From<ActivityError> for ResponseError {
    fn from(error: ActivityError) -> Self {
        ResponseError::Activity(error)
    }
}

impl From<AuthenticationError> for ResponseError {
    fn from(error: AuthenticationError) -> Self {
        ResponseError::Authentication(error)
    }
}

impl From<TeamError> for ResponseError {
    fn from(error: TeamError) -> Self {
        ResponseError::Team(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitiesResponse {
    pub message: Option<String>,
    pub data: Option<Value>,
    pub error: Option<ResponseError>,
}

impl ActivitiesResponse {
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
pub async fn get_activities(token: RawToken) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(ActivitiesResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let activities_params: Vec<(&str, &DatabaseValue)> = vec![("assigned_to", &user_id)];
    let my_activities =
        match find_all_unarchived_resources_where_fields!(Activity, activities_params).await {
            Ok(activities) => activities,
            Err(err) => {
                println!("Error fetching activities: {:?}", err);
                return status::Custom(
                    Status::NotFound,
                    serde_json::to_value(ActivitiesResponse::error(
                        ActivityError::ActivityNotFound,
                        ActivityError::ActivityNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };

    let team_id = DatabaseValue::String(token_value.user_id.clone());
    let team_activities_params: Vec<(&str, &DatabaseValue)> = vec![("team_id", &team_id)];
    let team_activities =
        match find_all_unarchived_resources_where_fields!(Activity, team_activities_params).await {
            Ok(activities) => activities,
            Err(err) => {
                println!("Error fetching activities: {:?}", err);
                return status::Custom(
                    Status::NotFound,
                    serde_json::to_value(ActivitiesResponse::error(
                        ActivityError::ActivityNotFound,
                        ActivityError::ActivityNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };

    let activities = vec![my_activities, team_activities];

    status::Custom(
        Status::Ok,
        serde_json::to_value(ActivitiesResponse::success(
            serde_json::to_value(activities).unwrap(),
            Some("Activities fetched successfully".to_string()),
        ))
        .unwrap(),
    )
}

#[get("/<activity_id>")]
pub async fn get_activity(token: RawToken, activity_id: &str) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(ActivitiesResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let activity_id_value = DatabaseValue::String(activity_id.to_string());
    let find_params = vec![("id", &activity_id_value), ("assigned_to", &user_id)];

    match find_one_unarchived_resource_where_fields!(Activity, find_params).await {
        Ok(activity) => status::Custom(
            Status::Ok,
            serde_json::to_value(ActivitiesResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/<activity_id>/complete")]
pub async fn complete_activity(token: RawToken, activity_id: &str) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(ActivitiesResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_value = DatabaseValue::String(activity_id.to_string());
    let find_params = vec![("id", &activity_id_value)];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, find_params).await {
        Ok(activity) => {
            if activity.assigned_to.clone().unwrap() != token_value.user_id {
                return status::Custom(
                    Status::Forbidden,
                    serde_json::to_value(ActivitiesResponse::error(
                        ActivityError::ActivityNotFound,
                        ActivityError::ActivityNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
            activity
        }
        Err(err) => {
            println!("Error fetching activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id = DatabaseValue::String(activity_id_value.to_string());
    let complete_params: Vec<(&str, DatabaseValue)> = vec![(
        "completed_at",
        DatabaseValue::DateTime(OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap()),
    )];

    match update_resource!(Activity, activity_id, complete_params).await {
        Ok(activity) => {
            let activity_response = ActivitiesResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity completed successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error completing activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/<activity_id>/reopen")]
pub async fn reopen_activity(token: RawToken, activity_id: &str) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(ActivitiesResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_value = DatabaseValue::String(activity_id.to_string());
    let find_params = vec![("id", &activity_id_value)];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, find_params).await {
        Ok(activity) => {
            if activity.assigned_to.clone().unwrap() != token_value.user_id {
                return status::Custom(
                    Status::Forbidden,
                    serde_json::to_value(ActivitiesResponse::error(
                        ActivityError::ActivityNotFound,
                        ActivityError::ActivityNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
            activity
        }
        Err(err) => {
            println!("Error fetching activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id = DatabaseValue::String(activity_id_value.to_string());
    let reopen_params: Vec<(&str, DatabaseValue)> = vec![("completed_at", DatabaseValue::None)];

    match update_resource!(Activity, activity_id, reopen_params).await {
        Ok(activity) => {
            let activity_response = ActivitiesResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity reopened successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error reopening activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/<activity_id>/pause")]
pub async fn pause_activity(token: RawToken, activity_id: &str) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(ActivitiesResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_value = DatabaseValue::String(activity_id.to_string());
    let find_params = vec![("id", &activity_id_value)];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, find_params).await {
        Ok(activity) => {
            if activity.assigned_to.clone().unwrap() != token_value.user_id {
                return status::Custom(
                    Status::Forbidden,
                    serde_json::to_value(ActivitiesResponse::error(
                        ActivityError::ActivityNotFound,
                        ActivityError::ActivityNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
            activity
        }
        Err(err) => {
            println!("Error fetching activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id = DatabaseValue::String(activity_id_value.to_string());
    let pause_params: Vec<(&str, DatabaseValue)> = vec![(
        "paused_at",
        DatabaseValue::DateTime(OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap()),
    )];

    match update_resource!(Activity, activity_id, pause_params).await {
        Ok(activity) => {
            let activity_response = ActivitiesResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity paused successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error pausing activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/<activity_id>/resume")]
pub async fn resume_activity(token: RawToken, activity_id: &str) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(ActivitiesResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_value = DatabaseValue::String(activity_id.to_string());
    let find_params = vec![("id", &activity_id_value)];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, find_params).await {
        Ok(activity) => {
            if activity.assigned_to.clone().unwrap() != token_value.user_id {
                return status::Custom(
                    Status::Forbidden,
                    serde_json::to_value(ActivitiesResponse::error(
                        ActivityError::ActivityNotFound,
                        ActivityError::ActivityNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
            activity
        }
        Err(err) => {
            println!("Error fetching activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id = DatabaseValue::String(activity_id_value.to_string());
    let resume_params: Vec<(&str, DatabaseValue)> = vec![("paused_at", DatabaseValue::None)];

    match update_resource!(Activity, activity_id, resume_params).await {
        Ok(activity) => {
            let activity_response = ActivitiesResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity resumed successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error resuming activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(ActivitiesResponse::error(
                    ActivityError::ActivityNotFound,
                    ActivityError::ActivityNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}
