use std::sync::Arc;

use crate::db::EventRepository;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn EventRepository>,
}
