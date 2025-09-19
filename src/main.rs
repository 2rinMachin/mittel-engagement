use std::{env, sync::Arc};

use anyhow::anyhow;
use async_trait::async_trait;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use env_logger::Env;
use serde::Serialize;
use serde_json::json;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use thiserror::Error;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, sqlx::Type, Serialize, utoipa::ToSchema)]
#[sqlx(type_name = "event_kind", rename_all = "lowercase")]
pub enum EventKind {
    View,
    Like,
    Share,
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
struct Event {
    id: i64,
    user_id: Option<String>,
    post_id: String,
    kind: EventKind,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct CreateEventRequest {
    user_id: Option<String>,
    post_id: String,
    kind: EventKind,
}

#[async_trait]
trait EventRepository: Send + Sync {
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

#[derive(Clone)]
struct AppState {
    repo: Arc<dyn EventRepository>,
}

#[derive(Debug, Error)]
enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError(#[from] anyhow::Error),

    #[error("Bad Request")]
    BadRequest(Option<String>),

    #[error("Conflict")]
    Conflict(Option<String>),
}

#[derive(Debug, Clone, Serialize)]
struct StatusResponse {
    status: u16,
    title: String,
    detail: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InternalServerError(ref cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());

                let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                let response = StatusResponse {
                    status: status_code.as_u16(),
                    title: self.to_string(),
                    detail: None,
                };

                (status_code, Json(response)).into_response()
            }
            Self::BadRequest(ref detail) => {
                let status_code = StatusCode::BAD_REQUEST;
                let response = StatusResponse {
                    status: status_code.as_u16(),
                    title: self.to_string(),
                    detail: detail.clone(),
                };

                (status_code, Json(response)).into_response()
            }
            Self::Conflict(ref detail) => {
                let status_code = StatusCode::CONFLICT;
                let response = StatusResponse {
                    status: status_code.as_u16(),
                    title: self.to_string(),
                    detail: detail.clone(),
                };

                (status_code, Json(response)).into_response()
            }
        }
    }
}

type ApiResult<T> = Result<T, ApiError>;

#[utoipa::path(get, path = "/", description = "Returns a status message", responses( (status = OK)))]
async fn get_hello(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(json!({ "hello": "world" }))
}

#[utoipa::path(get, path = "/events", description = "Returns all events", responses((status = OK, body = [Event])))]
async fn get_all_events(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Event>>> {
    let events = state.repo.find_all_events().await?;
    Ok(Json(events))
}

#[derive(OpenApi)]
#[openapi(info(
    title = "Mittel Engagement",
    description = "API for managing engagement on the Mittel blogging platform.",
    version = "0.1.0",
))]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let postgres = MySql::new(&database_url).await?;

    let state = Arc::new(AppState {
        repo: Arc::new(postgres),
    });

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_hello))
        .routes(routes!(get_all_events))
        .split_for_parts();

    let swagger = SwaggerUi::new("/docs").url("/openapi.json", api);

    let app = router.merge(swagger).with_state(state);

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let addr = format!("{host}:{port}");

    let listener = TcpListener::bind(addr).await?;

    log::info!("Listening at {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
