use async_trait::async_trait;

#[async_trait]
pub trait PostsApi: Send + Sync {
    async fn validate_post_id(&self, post_id: &str) -> anyhow::Result<bool>;
}

#[derive(Debug, Clone)]
pub struct PostsMicroserviceClient {
    secret_token: String,
}

impl PostsMicroserviceClient {
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            secret_token: token.into(),
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
