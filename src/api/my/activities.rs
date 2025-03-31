use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::find_all_unarchived_resources_where_fields;
use crate::models::activity::{Activity, ActivityError};
use crate::models::authentication::AuthenticationError;
use crate::models::team::TeamError;
use rocket::http::Status;
use rocket::response::status;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
