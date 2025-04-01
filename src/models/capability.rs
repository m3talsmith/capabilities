use crate::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub user: User,
    pub skill: String,
    pub level: i32,
    pub available: bool,
}
