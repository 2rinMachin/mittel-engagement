use anyhow::anyhow;
use async_trait::async_trait;
use num_traits::cast::ToPrimitive;
use serde::Deserialize;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use utoipa::ToSchema;

use crate::domain::{Device, Event, EventKind, EventSummary};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct DeviceRequest {
    pub os: String,
    pub browser: String,
    pub screen_resolution: String,
    pub language: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateEventRequest {
    pub post_id: String,
    pub kind: EventKind,
    pub device: Option<DeviceRequest>,
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_events(
        &self,
        user_id: &Option<String>,
        post_id: &Option<String>,
    ) -> anyhow::Result<Vec<Event>>;

    async fn find_event_summary(&self, post_id: &str) -> anyhow::Result<EventSummary>;

    async fn create_event(
        &self,
        event: CreateEventRequest,
        user_id: Option<String>,
    ) -> anyhow::Result<i64>;

    async fn find_devices(&self) -> anyhow::Result<Vec<Device>>;
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

        sqlx::migrate!().run(&pool).await?;

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
                device_id,
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

    async fn find_event_summary(&self, post_id: &str) -> anyhow::Result<EventSummary> {
        let rec = sqlx::query!(
            r#"
            select
                coalesce(sum(kind = 'view'), 0) as views,
                coalesce(sum(kind = 'like'), 0) as likes,
                coalesce(sum(kind = 'share'), 0) as shares
            from events
            where ? is null or post_id = ?
            "#,
            post_id,
            post_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?;

        Ok(EventSummary {
            views: rec.views.to_usize().unwrap(),
            likes: rec.likes.to_usize().unwrap(),
            shares: rec.shares.to_usize().unwrap(),
        })
    }

    async fn create_event(
        &self,
        event: CreateEventRequest,
        user_id: Option<String>,
    ) -> anyhow::Result<i64> {
        let device_id: Option<i64> = if let Some(device) = event.device {
            let rec = sqlx::query_as!(
                Event,
                r#"
                insert into devices (os, browser, screen_resolution, language)
                values (?, ?, ?, ?)
                on duplicate key update id = last_insert_id(id)
                "#,
                device.os,
                device.browser,
                device.screen_resolution,
                device.language,
            )
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow!(e))?;

            Some(rec.last_insert_id().try_into().unwrap())
        } else {
            None
        };

        let rec = sqlx::query!(
            r#"
            insert into events (user_id, device_id, post_id, kind, timestamp)
            values (?, ?, ?, ?, ?)
            "#,
            user_id,
            device_id,
            event.post_id,
            event.kind,
            chrono::offset::Utc::now()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?;

        Ok(rec.last_insert_id().try_into().unwrap())
    }

    async fn find_devices(&self) -> anyhow::Result<Vec<Device>> {
        Ok(sqlx::query_as!(Device, "select * from devices")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| anyhow!(e))?)
    }
}
