use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::activity::{Activity, ActivityError};
use crate::models::authentication::AuthenticationError;
use crate::models::invitation::{Invitation, InvitationError};
use crate::models::team::{Team, TeamError};
use crate::models::team_role::TeamRole;
use crate::models::user::{User, UserError};
use crate::{
    delete_resource_where_fields, find_all_unarchived_resources_where_fields,
    find_one_unarchived_resource_where_fields, insert_resource, update_resource,
};
use futures::future::try_join_all;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::{Json, Value};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    Authentication(AuthenticationError),
    User(UserError),
    Team(TeamError),
    Invitation(InvitationError),
    Activity(ActivityError),
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

impl From<TeamError> for ResponseError {
    fn from(error: TeamError) -> Self {
        ResponseError::Team(error)
    }
}

impl From<InvitationError> for ResponseError {
    fn from(error: InvitationError) -> Self {
        ResponseError::Invitation(error)
    }
}

impl From<ActivityError> for ResponseError {
    fn from(error: ActivityError) -> Self {
        ResponseError::Activity(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamsResponse {
    pub error: Option<ResponseError>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl TeamsResponse {
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
pub async fn get_teams(token: RawToken) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let find_user_params = vec![("id", &user_id)];
    let _ = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let owned_teams_params = vec![("owner_id", &user_id)];
    match find_all_unarchived_resources_where_fields!(Team, owned_teams_params).await {
        Ok(teams) => status::Custom(
            Status::Ok,
            serde_json::to_value(TeamsResponse::success(
                serde_json::to_value(teams).unwrap(),
                Some("Teams fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error finding teams: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitationResponse {
    pub invitation: Invitation,
    pub user: User,
    pub accepted: bool,
    pub rejected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamResponse {
    pub team: Team,
    pub users: Vec<User>,
    pub invitations: Vec<InvitationResponse>,
}

#[get("/<team_id>")]
pub async fn get_team(token: RawToken, team_id: String) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
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
                serde_json::to_value(TeamsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let users = vec![user];
    let invitations_params = vec![("team_id", &team_id)];
    let invitations =
        match find_all_unarchived_resources_where_fields!(Invitation, invitations_params).await {
            Ok(invitations) => invitations,
            Err(err) => {
                println!("Error finding invitations: {:?}", err);
                return status::Custom(
                    Status::NotFound,
                    serde_json::to_value(TeamsResponse::error(
                        TeamError::TeamNotFound,
                        TeamError::TeamNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };
    let invitation_responses = invitations.iter().map(|invitation| async move {
        let user_id = invitation.user_id.clone();
        let user_params = vec![("id", &user_id)];
        let user = match find_one_unarchived_resource_where_fields!(User, user_params).await {
            Ok(user) => user,
            Err(err) => {
                println!("Error finding user: {:?}", err);
                return Err(err);
            }
        };
        Ok(InvitationResponse {
            invitation: invitation.clone(),
            user,
            accepted: invitation.is_accepted(),
            rejected: invitation.is_rejected(),
        })
    });
    let invitations_response = try_join_all(invitation_responses).await.unwrap();

    let team_id_param = vec![("id", &team_id), ("owner_id", &token_value.user_id)];
    match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => status::Custom(
            Status::Ok,
            serde_json::to_value(TeamsResponse::success(
                serde_json::to_value(TeamResponse {
                    team,
                    users,
                    invitations: invitations_response,
                })
                .unwrap(),
                Some("Team fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    pub team_name: String,
    pub team_description: Option<String>,
}

#[post("/", data = "<team_data>")]
pub async fn create_team(
    token: RawToken,
    team_data: Json<CreateTeamRequest>,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let find_user_params = vec![("id", &user_id)];
    let _ = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    // Create the team
    let team_params = vec![
        (
            "owner_id",
            DatabaseValue::String(token_value.user_id.clone()),
        ),
        (
            "team_name",
            DatabaseValue::String(team_data.team_name.clone()),
        ),
        (
            "team_description",
            match &team_data.team_description {
                Some(desc) => DatabaseValue::String(desc.clone()),
                None => DatabaseValue::None,
            },
        ),
    ];

    let team = match insert_resource!(Team, team_params).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error creating team: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamCreationFailed,
                    TeamError::TeamCreationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_response = TeamsResponse::success(
        serde_json::to_value(team).unwrap(),
        Some("Team created successfully".to_string()),
    );

    status::Custom(
        Status::Created,
        serde_json::to_value(team_response).unwrap(),
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeamRequest {
    pub team_name: Option<String>,
    pub team_description: Option<String>,
}

#[put("/<team_id>", data = "<team_data>")]
pub async fn update_team(
    token: RawToken,
    team_id: String,
    team_data: Json<UpdateTeamRequest>,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let find_user_params = vec![("id", &user_id)];
    let _ = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![("id", &team_id)];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_update_params = vec![
        (
            "team_name",
            DatabaseValue::String(team_data.team_name.clone().unwrap()),
        ),
        (
            "team_description",
            DatabaseValue::String(team_data.team_description.clone().unwrap()),
        ),
    ];

    match update_resource!(Team, team_id, team_update_params).await {
        Ok(team) => {
            let team_response = TeamsResponse::success(
                serde_json::to_value(team).unwrap(),
                Some("Team updated successfully".to_string()),
            );

            status::Custom(Status::Ok, serde_json::to_value(team_response).unwrap())
        }
        Err(err) => {
            println!("Error updating team: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamUpdateFailed,
                    TeamError::TeamUpdateFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[delete("/<team_id>")]
pub async fn delete_team(token: RawToken, team_id: String) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let find_user_params = vec![("id", &user_id)];
    let _ = match find_one_unarchived_resource_where_fields!(User, find_user_params).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![("id", &team_id)];
    let team = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    match delete_resource_where_fields!(Team, team_id_param).await {
        Ok(_) => {
            let team_response = TeamsResponse::success(
                serde_json::to_value(team).unwrap(),
                Some("Team deleted successfully".to_string()),
            );

            status::Custom(Status::Ok, serde_json::to_value(team_response).unwrap())
        }
        Err(err) => {
            println!("Error deleting team: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamDeletionFailed,
                    TeamError::TeamDeletionFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitationRequest {
    pub user_id: String,
    pub team_role: TeamRole,
}

#[post("/<team_id>/invitations", data = "<invitation_data>")]
pub async fn create_invitation(
    token: RawToken,
    team_id: String,
    invitation_data: Json<InvitationRequest>,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![("id", &team_id)];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id = DatabaseValue::String(token_value.user_id.clone());
    let team_id = DatabaseValue::String(team_id.clone());
    let team_role = DatabaseValue::String(invitation_data.0.team_role.to_string());

    let invitation_params = vec![
        ("user_id", user_id),
        ("team_id", team_id),
        ("team_role", team_role),
    ];

    let invitation = match insert_resource!(Invitation, invitation_params).await {
        Ok(invitation) => invitation,
        Err(err) => {
            println!("Error creating invitation: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    InvitationError::InvitationCreationFailed,
                    InvitationError::InvitationCreationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let invitation_response = TeamsResponse::success(
        serde_json::to_value(invitation).unwrap(),
        Some("Invitation created successfully".to_string()),
    );

    status::Custom(
        Status::Created,
        serde_json::to_value(invitation_response).unwrap(),
    )
}

#[get("/<team_id>/activities")]
pub async fn get_team_activities(token: RawToken, team_id: &str) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![("team_id", DatabaseValue::String(team_id.to_string()))];
    match find_all_unarchived_resources_where_fields!(Activity, team_id_param).await {
        Ok(activities) => status::Custom(
            Status::Ok,
            serde_json::to_value(TeamsResponse::success(
                serde_json::to_value(activities).unwrap(),
                Some("Activities fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error fetching activities: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[get("/<team_id>/activities/<activity_id>")]
pub async fn get_team_activity(
    token: RawToken,
    team_id: &str,
    activity_id: &str,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_param = vec![
        ("id", DatabaseValue::String(activity_id.to_string())),
        ("team_id", DatabaseValue::String(team_id.to_string())),
    ];
    match find_one_unarchived_resource_where_fields!(Activity, activity_id_param).await {
        Ok(activity) => status::Custom(
            Status::Ok,
            serde_json::to_value(TeamsResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity fetched successfully".to_string()),
            ))
            .unwrap(),
        ),
        Err(err) => {
            println!("Error finding activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamActivityRequest {
    pub activity_name: String,
    pub activity_description: String,
}

#[post("/<team_id>/activities", data = "<activity_data>")]
pub async fn create_team_activity(
    token: RawToken,
    team_id: &str,
    activity_data: Json<CreateTeamActivityRequest>,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_params = vec![
        ("team_id", DatabaseValue::String(team_id.to_string())),
        (
            "activity_name",
            DatabaseValue::String(activity_data.activity_name.clone()),
        ),
        (
            "activity_description",
            DatabaseValue::String(activity_data.activity_description.clone()),
        ),
    ];

    let activity = match insert_resource!(Activity, activity_params).await {
        Ok(activity) => activity,
        Err(err) => {
            println!("Error creating activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    ActivityError::ActivityCreationFailed,
                    ActivityError::ActivityCreationFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_response = TeamsResponse::success(
        serde_json::to_value(activity).unwrap(),
        Some("Activity created successfully".to_string()),
    );

    status::Custom(
        Status::Created,
        serde_json::to_value(activity_response).unwrap(),
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTeamActivityRequest {
    pub activity_name: Option<String>,
    pub activity_description: Option<String>,
}

#[put("/<team_id>/activities/<activity_id>", data = "<activity_data>")]
pub async fn update_team_activity(
    token: RawToken,
    team_id: &str,
    activity_id: &str,
    activity_data: Json<UpdateTeamActivityRequest>,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![("id", DatabaseValue::String(team_id.to_string()))];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_params = vec![
        (
            "activity_name",
            DatabaseValue::String(activity_data.activity_name.clone().unwrap()),
        ),
        (
            "activity_description",
            DatabaseValue::String(activity_data.activity_description.clone().unwrap()),
        ),
    ];

    let activity_id_param = activity_id.to_string();
    match update_resource!(Activity, activity_id_param, activity_params).await {
        Ok(activity) => {
            let activity_response = TeamsResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity updated successfully".to_string()),
            );

            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error updating activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    ActivityError::ActivityUpdateFailed,
                    ActivityError::ActivityUpdateFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[delete("/<team_id>/activities/<activity_id>")]
pub async fn delete_team_activity(
    token: RawToken,
    team_id: &str,
    activity_id: &str,
) -> status::Custom<Value> {
    let _ = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_param = vec![
        ("id", activity_id.to_string()),
        ("team_id", team_id.to_string()),
    ];
    match delete_resource_where_fields!(Activity, activity_id_param).await {
        Ok(_) => {
            let activity_response = TeamsResponse::success(
                serde_json::to_value(activity_id).unwrap(),
                Some("Activity deleted successfully".to_string()),
            );

            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error deleting activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    ActivityError::ActivityDeletionFailed,
                    ActivityError::ActivityDeletionFailed.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignTeamActivityRequest {
    pub user_id: String,
}

#[post("/<team_id>/activities/<activity_id>/assign", data = "<assign_data>")]
pub async fn assign_team_activity(
    token: RawToken,
    team_id: &str,
    activity_id: &str,
    assign_data: Json<AssignTeamActivityRequest>,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![
        ("id", DatabaseValue::String(team_id.to_string())),
        (
            "user_id",
            DatabaseValue::String(token_value.user_id.clone()),
        ),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_param = vec![
        ("id", DatabaseValue::String(activity_id.to_string())),
        ("team_id", DatabaseValue::String(team_id.to_string())),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, activity_id_param).await {
        Ok(activity) => activity,
        Err(err) => {
            println!("Error finding activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let user_id_param = vec![("id", DatabaseValue::String(assign_data.user_id.clone()))];
    let _ = match find_one_unarchived_resource_where_fields!(User, user_id_param).await {
        Ok(user) => user,
        Err(err) => {
            println!("Error finding user: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![("id", DatabaseValue::String(team_id.to_string()))];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_activity_params = vec![
        ("team_id", DatabaseValue::String(team_id.to_string())),
        (
            "activity_id",
            DatabaseValue::String(activity_id.to_string()),
        ),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, team_activity_params).await {
        Ok(team_activity) => team_activity,
        Err(err) => {
            println!("Error finding team activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let assignment_params = vec![(
        "assigned_to",
        DatabaseValue::String(assign_data.user_id.clone()),
    )];
    let activity_id = activity_id.to_string();
    match update_resource!(Activity, activity_id, assignment_params).await {
        Ok(activity) => {
            let activity_response = TeamsResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity assigned successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error updating team activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/<team_id>/activities/<activity_id>/unassign")]
pub async fn unassign_team_activity(
    token: RawToken,
    team_id: &str,
    activity_id: &str,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![
        ("id", DatabaseValue::String(team_id.to_string())),
        (
            "user_id",
            DatabaseValue::String(token_value.user_id.clone()),
        ),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_param = vec![
        ("id", DatabaseValue::String(activity_id.to_string())),
        ("team_id", DatabaseValue::String(team_id.to_string())),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, activity_id_param).await {
        Ok(activity) => activity,
        Err(err) => {
            println!("Error finding activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let unassign_params = vec![("assigned_to", DatabaseValue::None)];
    let activity_id = activity_id.to_string();
    match update_resource!(Activity, activity_id, unassign_params).await {
        Ok(activity) => {
            let activity_response = TeamsResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity unassigned successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error unassigning activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/<team_id>/activities/<activity_id>/pause")]
pub async fn pause_team_activity(
    token: RawToken,
    team_id: &str,
    activity_id: &str,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![
        ("id", DatabaseValue::String(team_id.to_string())),
        (
            "user_id",
            DatabaseValue::String(token_value.user_id.clone()),
        ),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_param = vec![
        ("id", DatabaseValue::String(activity_id.to_string())),
        ("team_id", DatabaseValue::String(team_id.to_string())),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, activity_id_param).await {
        Ok(activity) => activity,
        Err(err) => {
            println!("Error finding activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let pause_params = vec![(
        "paused_at",
        DatabaseValue::DateTime(OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap()),
    )];
    let activity_id = activity_id.to_string();
    match update_resource!(Activity, activity_id, pause_params).await {
        Ok(activity) => {
            let activity_response = TeamsResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity paused successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error pausing activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/<team_id>/activities/<activity_id>/resume")]
pub async fn resume_team_activity(
    token: RawToken,
    team_id: &str,
    activity_id: &str,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![
        ("id", DatabaseValue::String(team_id.to_string())),
        (
            "user_id",
            DatabaseValue::String(token_value.user_id.clone()),
        ),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_param = vec![
        ("id", DatabaseValue::String(activity_id.to_string())),
        ("team_id", DatabaseValue::String(team_id.to_string())),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, activity_id_param).await {
        Ok(activity) => activity,
        Err(err) => {
            println!("Error finding activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let resume_params = vec![("paused_at", DatabaseValue::None)];
    let activity_id = activity_id.to_string();
    match update_resource!(Activity, activity_id, resume_params).await {
        Ok(activity) => {
            let activity_response = TeamsResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity resumed successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error resuming activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}

#[post("/<team_id>/activities/<activity_id>/reopen")]
pub async fn reopen_team_activity(
    token: RawToken,
    team_id: &str,
    activity_id: &str,
) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(TeamsResponse::error(
                    AuthenticationError::InvalidToken,
                    AuthenticationError::InvalidToken.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let team_id_param = vec![
        ("id", DatabaseValue::String(team_id.to_string())),
        (
            "user_id",
            DatabaseValue::String(token_value.user_id.clone()),
        ),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let activity_id_param = vec![
        ("id", DatabaseValue::String(activity_id.to_string())),
        ("team_id", DatabaseValue::String(team_id.to_string())),
    ];
    let _ = match find_one_unarchived_resource_where_fields!(Activity, activity_id_param).await {
        Ok(activity) => activity,
        Err(err) => {
            println!("Error finding activity: {:?}", err);
            return status::Custom(
                Status::NotFound,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let reopen_params = vec![("completed_at", DatabaseValue::None)];
    let activity_id = activity_id.to_string();
    match update_resource!(Activity, activity_id, reopen_params).await {
        Ok(activity) => {
            let activity_response = TeamsResponse::success(
                serde_json::to_value(activity).unwrap(),
                Some("Activity reopened successfully".to_string()),
            );
            status::Custom(Status::Ok, serde_json::to_value(activity_response).unwrap())
        }
        Err(err) => {
            println!("Error reopening activity: {:?}", err);
            return status::Custom(
                Status::InternalServerError,
                serde_json::to_value(TeamsResponse::error(
                    TeamError::TeamNotFound,
                    TeamError::TeamNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    }
}
