use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "team_role", rename_all = "snake_case")]
pub enum TeamRole {
    Admin,
    Manager,
    Member,
}

impl std::fmt::Display for TeamRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamRole::Admin => write!(f, "admin"),
            TeamRole::Manager => write!(f, "manager"),
            TeamRole::Member => write!(f, "member"),
        }
    }
}

impl FromStr for TeamRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(TeamRole::Admin),
            "manager" => Ok(TeamRole::Manager),
            "member" => Ok(TeamRole::Member),
            _ => Err("Invalid team role".to_string()),
        }
    }
}

impl From<TeamRole> for String {
    fn from(role: TeamRole) -> Self {
        role.to_string()
    }
}

impl From<String> for TeamRole {
    fn from(s: String) -> Self {
        TeamRole::from_str(&s).unwrap()
    }
}
