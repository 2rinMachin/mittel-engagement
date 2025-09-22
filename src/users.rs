use async_trait::async_trait;
use thiserror::Error;

use crate::http::ApiError;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Error)]
pub enum FetchUserError {
    #[error("Invalid authorization")]
    InvalidAuthorization,

    #[error("Unknown: {_0}")]
    Unknown(anyhow::Error),
}

impl From<FetchUserError> for ApiError {
    fn from(value: FetchUserError) -> Self {
        match value {
            FetchUserError::InvalidAuthorization => Self::Unauthorized(None),
            FetchUserError::Unknown(e) => Self::InternalServerError(e),
        }
    }
}

#[async_trait]
pub trait UsersApi: Send + Sync {
    async fn validate_auth(&self, authorization: &str) -> anyhow::Result<bool>;

    async fn fetch_user(&self, authorization: &str) -> Result<Option<User>, FetchUserError>;
}

#[derive(Debug, Clone)]
pub struct UsersMicroserviceClient {
    secret_token: String,
}

impl UsersMicroserviceClient {
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            secret_token: token.into(),
        }
    }
}

#[async_trait]
impl UsersApi for UsersMicroserviceClient {
    async fn validate_auth(&self, authorization: &str) -> anyhow::Result<bool> {
        todo!("apurate joaquin");
    }

    async fn fetch_user(&self, authorization: &str) -> Result<Option<User>, FetchUserError> {
        todo!("apurate joaquin");
    }
}

#[derive(Debug, Clone)]
pub struct MockUsersClient;

#[async_trait]
impl UsersApi for MockUsersClient {
    async fn validate_auth(&self, authorization: &str) -> anyhow::Result<bool> {
        Ok(authorization.len() >= 10)
    }

    async fn fetch_user(&self, authorization: &str) -> Result<Option<User>, FetchUserError> {
        if authorization.len() >= 10 {
            Ok(Some(User {
                id: "1234567890".to_owned(),
            }))
        } else {
            Ok(None)
        }
    }
}
