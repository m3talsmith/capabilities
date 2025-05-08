use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::activity::Activity;
use crate::models::authentication::AuthenticationError;
use crate::models::capability::Capability;
use crate::models::invitation::{Invitation, InvitationError};
use crate::models::team::{Team, TeamError};
use crate::models::user::{User, UserError};
use crate::models::user_skill::UserSkill;
use crate::{
    find_all_unarchived_resources_where_fields, find_one_unarchived_resource_where_fields,
    join_all_resources_where_fields_on,
};
use futures::future::try_join_all;
use rocket::http::Status;
use rocket::response::status;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Error;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    Authentication(AuthenticationError),
    Invitation(InvitationError),
    Team(TeamError),
    User(UserError),
}

impl From<AuthenticationError> for ResponseError {
    fn from(error: AuthenticationError) -> Self {
        ResponseError::Authentication(error)
    }
}

impl From<InvitationError> for ResponseError {
    fn from(error: InvitationError) -> Self {
        ResponseError::Invitation(error)
    }
}

impl From<TeamError> for ResponseError {
    fn from(error: TeamError) -> Self {
        ResponseError::Team(error)
    }
}

impl From<UserError> for ResponseError {
    fn from(error: UserError) -> Self {
        ResponseError::User(error)
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

    let owned_teams_params = vec![("owner_id", &token_value.user_id)];
    let owned_teams =
        match find_all_unarchived_resources_where_fields!(Team, owned_teams_params).await {
            Ok(teams) => teams,
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
        };

    let invitations_params = vec![("user_id", &token_value.user_id)];
    let invitations =
        match find_all_unarchived_resources_where_fields!(Invitation, invitations_params).await {
            Ok(invitations) => invitations,
            Err(err) => {
                println!("Error finding invitations: {:?}", err);
                return status::Custom(
                    Status::NotFound,
                    serde_json::to_value(TeamsResponse::error(
                        InvitationError::InvitationNotFound,
                        InvitationError::InvitationNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };

    let team_futures = invitations
        .iter()
        .map(|invitation| async move {
            let team_id = invitation.team_id.clone().unwrap();
            let team_id_param = vec![("id", &team_id)];
            find_one_unarchived_resource_where_fields!(Team, team_id_param).await
        })
        .collect::<Vec<_>>();

    let invited_teams = try_join_all(team_futures).await.unwrap_or_default();

    let mut teams = Vec::new();
    teams.extend(owned_teams);
    teams.extend(invited_teams);

    status::Custom(
        Status::Ok,
        serde_json::to_value(TeamsResponse::success(
            serde_json::to_value(teams).unwrap(),
            Some("Teams fetched successfully".to_string()),
        ))
        .unwrap(),
    )
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
    pub invitations: Vec<InvitationResponse>,
    pub capabilities: HashMap<String, Vec<Capability>>,
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

    let invitation_params = vec![("user_id", &token_value.user_id), ("team_id", &team_id)];
    let invitation =
        match find_one_unarchived_resource_where_fields!(Invitation, invitation_params).await {
            Ok(invitation) => Some(invitation),
            Err(err) => {
                println!("Error finding invitation: {:?}", err);
                None
            }
        };

    let owned_team_params = vec![("owner_id", &token_value.user_id), ("id", &team_id)];
    let owned_team = match find_one_unarchived_resource_where_fields!(Team, owned_team_params).await
    {
        Ok(team) => Some(team),
        Err(err) => {
            println!("Error finding team: {:?}", err);
            None
        }
    };

    let mut team: Option<Team> = None;

    if invitation.is_some() {
        let team_id_param = vec![("id", &team_id)];
        team = match find_one_unarchived_resource_where_fields!(Team, team_id_param).await {
            Ok(team) => Some(team),
            Err(err) => {
                println!("Error finding team: {:?}", err);
                None
            }
        };
    } else if owned_team.is_some() {
        team = owned_team;
    }

    match team {
        Some(team) => match get_team_response(team.id.unwrap()).await {
            Ok(team_response) => status::Custom(
                Status::Ok,
                serde_json::to_value(TeamsResponse::success(
                    serde_json::to_value(team_response).unwrap(),
                    Some("Team fetched successfully".to_string()),
                ))
                .unwrap(),
            ),
            Err(err) => {
                println!("Error getting team response: {:?}", err);
                return status::Custom(
                    Status::InternalServerError,
                    serde_json::to_value(TeamsResponse::error(
                        TeamError::TeamNotFound,
                        TeamError::TeamNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
        },
        None => status::Custom(
            Status::NotFound,
            serde_json::to_value(TeamsResponse::error(
                TeamError::TeamNotFound,
                TeamError::TeamNotFound.to_string(),
            ))
            .unwrap(),
        ),
    }
}

async fn get_team_response(team_id: String) -> Result<TeamResponse, Error> {
    let team_params = vec![("id", &team_id)];
    let team = match find_one_unarchived_resource_where_fields!(Team, team_params).await {
        Ok(team) => team,
        Err(err) => {
            println!("Error finding team: {:?}", err);
            return Err(err);
        }
    };
    let invitations_params = vec![("team_id", &team_id)];
    let invitations =
        match find_all_unarchived_resources_where_fields!(Invitation, invitations_params).await {
            Ok(invitations) => invitations,
            Err(err) => {
                println!("Error finding invitations: {:?}", err);
                return Err(err);
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

    let activity_params = vec![("team_id", &team_id)];
    let activities =
        match find_all_unarchived_resources_where_fields!(Activity, activity_params).await {
            Ok(activities) => activities,
            Err(err) => {
                println!("Error finding activities: {:?}", err);
                return Err(err);
            }
        };

    let mut capabilities: HashMap<String, Vec<Capability>> = HashMap::new();

    let users_params = vec![("teams.team_id", &team_id)];
    let users = join_all_resources_where_fields_on!(User, Team, users_params).await?;
    for user in users {
        let user_id = user.id.clone().unwrap();
        let skills_params = vec![("user_id", DatabaseValue::String(user_id.clone()))];
        let skills =
            match find_all_unarchived_resources_where_fields!(UserSkill, skills_params).await {
                Ok(skills) => skills,
                Err(err) => {
                    println!("Error finding skills: {:?}", err);
                    return Err(err);
                }
            };

        for skill in skills {
            let skill_name = skill.skill_name.clone().unwrap().to_lowercase();
            let capability = Capability {
                user: user.clone(),
                skill: skill_name.clone(),
                level: skill.skill_level.unwrap(),
                available: !activities
                    .iter()
                    .any(|activity| activity.assigned_to.clone().unwrap() == user_id),
            };
            capabilities
                .entry(skill_name)
                .or_insert(Vec::new())
                .push(capability);
        }
    }

    Ok(TeamResponse {
        team,
        invitations: invitations_response,
        capabilities,
    })
}
