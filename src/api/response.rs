use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseError {
    NotFound,
    Unauthorized,
    Forbidden,
    Failed,
    BadRequest,
    InternalServerError,
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::NotFound => write!(f, "Not Found"),
            ResponseError::Unauthorized => write!(f, "Unauthorized"),
            ResponseError::Forbidden => write!(f, "Forbidden"),
            ResponseError::Failed => write!(f, "Failed"),
            ResponseError::BadRequest => write!(f, "Bad Request"),
            ResponseError::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub error: Option<String>,
    pub message: Option<String>,
    pub data: Option<Value>,
}

impl Response {
    #[allow(dead_code)]
    pub fn success(data: Value, message: Option<String>) -> Self {
        Self {
            error: None,
            message: message,
            data: Some(data),
        }
    }

    #[allow(dead_code)]
    pub fn error(error: Error, message: String) -> Self {
        Self {
            error: Some(error.to_string()),
            message: Some(message),
            data: None,
        }
    }
}
