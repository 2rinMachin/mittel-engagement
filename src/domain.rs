use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "event_kind", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
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

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct Device {
    pub id: i64,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub language: Option<String>,
    pub screen_resolution: Option<String>,
}
