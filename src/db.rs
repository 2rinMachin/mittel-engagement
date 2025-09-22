use anyhow::anyhow;
use async_trait::async_trait;
use serde::Deserialize;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::domain::{Event, EventKind};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateEventRequest {
    pub post_id: String,
    pub kind: EventKind,
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_events(
        &self,
        user_id: &Option<String>,
        post_id: &Option<String>,
    ) -> anyhow::Result<Vec<Event>>;

    async fn create_event(
        &self,
        event: CreateEventRequest,
        user_id: Option<String>,
    ) -> anyhow::Result<i64>;
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
    async fn find_events(
        &self,
        user_id: &Option<String>,
        post_id: &Option<String>,
    ) -> anyhow::Result<Vec<Event>> {
        Ok(sqlx::query_as!(
            Event,
            r#"
            select
                id,
                user_id,
                post_id,
                kind as "kind: EventKind",
                timestamp
            from events
            where (? is null or user_id = ?)
                and (? is null or post_id = ?)
            "#,
            user_id,
            user_id,
            post_id,
            post_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?)
    }

    async fn create_event(
        &self,
        event: CreateEventRequest,
        user_id: Option<String>,
    ) -> anyhow::Result<i64> {
        let rec = sqlx::query!(
            r#"
            insert into events (user_id, post_id, kind, timestamp)
            values (?, ?, ?, ?)
            "#,
            user_id,
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
