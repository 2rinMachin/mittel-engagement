use std::collections::HashMap;

use anyhow::anyhow;
use async_trait::async_trait;
use reqwest::{Client, IntoUrl, StatusCode, Url};

#[async_trait]
pub trait PostsApi: Send + Sync {
    async fn validate_post_id(&self, post_id: &str) -> anyhow::Result<bool>;
}

#[derive(Debug, Clone)]
pub struct PostsMicroserviceClient {
    base_url: Url,
    client: Client,
}

impl PostsMicroserviceClient {
    pub fn new(base_url: impl IntoUrl) -> Self {
        Self {
            base_url: base_url.into_url().unwrap(),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl PostsApi for PostsMicroserviceClient {
    async fn validate_post_id(&self, post_id: &str) -> anyhow::Result<bool> {
        let url = self
            .base_url
            .join("/articles/")
            .and_then(|url| url.join(post_id))
            .unwrap();

        let res = self.client.get(url).send().await.map_err(|e| anyhow!(e))?;

        Ok(res.status() == StatusCode::OK)
    }
}

#[derive(Debug, Clone)]
pub struct MockPostsClient;

#[async_trait]
impl PostsApi for MockPostsClient {
    async fn validate_post_id(&self, post_id: &str) -> anyhow::Result<bool> {
        Ok(post_id.len() >= 10)
    }
}
