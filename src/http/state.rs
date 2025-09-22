use std::sync::Arc;

use crate::{db::EventRepository, posts::PostsApi, users::UsersApi};

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn EventRepository>,
    pub users: Arc<dyn UsersApi>,
    pub posts: Arc<dyn PostsApi>,
}
