use std::collections::HashMap;

use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::{Client, IntoUrl, StatusCode, Url};
use serde::Deserialize;
use thiserror::Error;

use crate::http::ApiError;

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Error)]
pub enum FetchUserError {
    #[error("Invalid authorization")]
    InvalidAuthorization,

    #[error("Unknown: {_0}")]
    Unknown(#[from] anyhow::Error),
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
    async fn fetch_user(&self, authorization: &str) -> Result<Option<User>, FetchUserError>;
}

#[derive(Debug, Clone)]
pub struct UsersMicroserviceClient {
    base_url: Url,
    client: Client,
}

impl UsersMicroserviceClient {
    pub fn new(base_url: impl IntoUrl) -> Self {
        Self {
            base_url: base_url.into_url().unwrap(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl UsersApi for UsersMicroserviceClient {
    async fn fetch_user(&self, authorization: &str) -> Result<Option<User>, FetchUserError> {
        let url = self.base_url.join("/introspect").unwrap();

        let mut body = HashMap::new();
        println!("auth: {authorization}");
        body.insert("token", authorization);

        let res = self
            .client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!(e))?;

        if res.status() == StatusCode::BAD_REQUEST {
            Ok(None)
        } else {
            let user = res.json().await.map_err(|e| anyhow!(e))?;
            Ok(Some(user))
        }
    }
}

#[derive(Debug, Clone)]
pub struct MockUsersClient;

#[async_trait]
impl UsersApi for MockUsersClient {
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
