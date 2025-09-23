use async_trait::async_trait;
use axum::http::HeaderMap;
use reqwest::{Client, ClientBuilder, IntoUrl, Url};
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
    base_url: Url,
    client: Client,
}

impl UsersMicroserviceClient {
    pub fn new(base_url: impl IntoUrl, token: impl Into<String>) -> Self {
        let mut client_headers = HeaderMap::new();
        client_headers.insert("X-Secret-Token", token.into().parse().unwrap());

        Self {
            base_url: base_url.into_url().unwrap(),
            client: ClientBuilder::new()
                .default_headers(client_headers)
                .build()
                .unwrap(),
        }
    }
}

#[async_trait]
impl UsersApi for UsersMicroserviceClient {
    async fn validate_auth(&self, authorization: &str) -> anyhow::Result<bool> {
        let url = self.base_url.join("/users/");

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
