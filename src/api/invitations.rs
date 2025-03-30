use crate::api::token::{validate_token, RawToken};
use crate::database::values::DatabaseValue;
use crate::models::authentication::AuthenticationError;
use crate::models::invitation::{Invitation, InvitationError};
use crate::models::team::TeamError;
use crate::models::user::{User, UserError};
use crate::{
    find_all_unarchived_resources_where_fields, find_one_unarchived_resource_where_fields,
};
use rocket::http::Status;
use rocket::response::status;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
pub struct InvitationsResponse {
    pub error: Option<ResponseError>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl InvitationsResponse {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitationResponse {
    pub invitation: Invitation,
    pub user: User,
    pub accepted: bool,
    pub rejected: bool,
}

#[get("/")]
pub async fn get_invitations(token: RawToken) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(InvitationsResponse::error(
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
                serde_json::to_value(InvitationsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let invitations_params = vec![("user_id", &user_id)];
    let invitations =
        match find_all_unarchived_resources_where_fields!(Invitation, invitations_params).await {
            Ok(invitations) => invitations,
            Err(err) => {
                println!("Error finding invitations: {:?}", err);
                return status::Custom(
                    Status::NotFound,
                    serde_json::to_value(InvitationsResponse::error(
                        InvitationError::InvitationNotFound,
                        InvitationError::InvitationNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };
    let invitations_response = invitations
        .iter()
        .map(|invitation| {
            let user = user.clone();
            InvitationResponse {
                invitation: invitation.clone(),
                user,
                accepted: invitation.is_accepted(),
                rejected: invitation.is_rejected(),
            }
        })
        .collect::<Vec<InvitationResponse>>();
    let invitations_response = InvitationsResponse::success(
        serde_json::to_value(invitations_response).unwrap(),
        Some("Invitations fetched successfully".to_string()),
    );
    status::Custom(
        Status::Ok,
        serde_json::to_value(invitations_response).unwrap(),
    )
}

#[get("/<invitation_id>")]
pub async fn get_invitation(token: RawToken, invitation_id: String) -> status::Custom<Value> {
    let token_value = match validate_token(token).await {
        Ok(token) => token,
        Err(err) => {
            println!("Error validating token: {:?}", err);
            return status::Custom(
                Status::Unauthorized,
                serde_json::to_value(InvitationsResponse::error(
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
                serde_json::to_value(InvitationsResponse::error(
                    UserError::UserNotFound,
                    UserError::UserNotFound.to_string(),
                ))
                .unwrap(),
            );
        }
    };

    let invitation_params = vec![("id", &invitation_id), ("user_id", &token_value.user_id)];
    let invitation =
        match find_one_unarchived_resource_where_fields!(Invitation, invitation_params).await {
            Ok(invitation) => invitation,
            Err(err) => {
                println!("Error finding invitation: {:?}", err);
                return status::Custom(
                    Status::NotFound,
                    serde_json::to_value(InvitationsResponse::error(
                        InvitationError::InvitationNotFound,
                        InvitationError::InvitationNotFound.to_string(),
                    ))
                    .unwrap(),
                );
            }
        };

    let invitation_response = InvitationResponse {
        invitation: invitation.clone(),
        user,
        accepted: invitation.is_accepted(),
        rejected: invitation.is_rejected(),
    };

    let invitations_response = InvitationsResponse::success(
        serde_json::to_value(invitation_response).unwrap(),
        Some("Invitation fetched successfully".to_string()),
    );

    status::Custom(
        Status::Ok,
        serde_json::to_value(invitations_response).unwrap(),
    )
}
