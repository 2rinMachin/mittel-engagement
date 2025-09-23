use async_trait::async_trait;
use axum::http::HeaderMap;
use reqwest::{Client, ClientBuilder, IntoUrl, Url};

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
impl PostsApi for PostsMicroserviceClient {
    async fn validate_post_id(&self, post_id: &str) -> anyhow::Result<bool> {
        todo!("apurate diego");
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
