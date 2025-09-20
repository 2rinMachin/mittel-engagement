use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::domain::{Event, EventKind};

#[derive(Debug, Clone)]
pub struct CreateEventRequest {
    user_id: Option<String>,
    post_id: String,
    kind: EventKind,
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_all_events(&self) -> anyhow::Result<Vec<Event>>;

    async fn find_event_by_id(&self, id: i64) -> anyhow::Result<Option<Event>>;

    async fn create_event(&self, event: CreateEventRequest) -> anyhow::Result<i64>;
}

#[derive(Clone)]
pub struct MySql {
    pool: MySqlPool,
}

impl MySql {
    pub async fn new(database_url: &str) -> sqlx::Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl EventRepository for MySql {
    async fn find_all_events(&self) -> anyhow::Result<Vec<Event>> {
        Ok(sqlx::query_as!(
            Event,
            r#"
            SELECT
                id,
                user_id,
                post_id,
                kind as "kind: EventKind",
                timestamp
            FROM events
            ORDER BY timestamp
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?)
    }

    async fn find_event_by_id(&self, id: i64) -> anyhow::Result<Option<Event>> {
        Ok(sqlx::query_as!(
            Event,
            r#"
            select
                id,
                user_id,
                post_id,
                kind as "kind: EventKind",
                timestamp
            from events where id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?)
    }

    async fn create_event(&self, event: CreateEventRequest) -> anyhow::Result<i64> {
        let rec = sqlx::query!(
            r#"
            insert into events (user_id, post_id, kind, timestamp)
            values (?, ?, ?, ?)
            "#,
            event.user_id,
            event.post_id,
            event.kind,
            chrono::offset::Utc::now()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?;

        Ok(rec.last_insert_id().try_into().unwrap())
    }
}
