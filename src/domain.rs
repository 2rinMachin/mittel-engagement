use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, sqlx::Type, Serialize, utoipa::ToSchema)]
#[sqlx(type_name = "event_kind", rename_all = "lowercase")]
pub enum EventKind {
    View,
    Like,
    Share,
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct Event {
    pub id: i64,
    pub user_id: Option<String>,
    pub post_id: String,
    pub kind: EventKind,
    pub timestamp: DateTime<Utc>,
}
