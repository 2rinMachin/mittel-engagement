pub mod extractors;
pub mod middleware;
pub mod routes;
pub mod state;

use std::sync::Arc;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Serializer};
use thiserror::Error;
use utoipa::ToSchema;

use crate::http::state::AppState;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError(#[from] anyhow::Error),

    #[error("Bad Request")]
    BadRequest(Option<String>),

    #[error("Conflict")]
    Conflict(Option<String>),

    #[error("Unauthorized")]
    Unauthorized(Option<String>),
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct StatusResponse {
    #[serde(serialize_with = "status_as_u16")]
    #[schema(value_type = u16, examples(200, 400, 418))]
    status: StatusCode,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

impl StatusResponse {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            title: status.to_string(),
            detail: None,
        }
    }

    pub fn with_detail(status: StatusCode, detail: Option<String>) -> Self {
        Self {
            status,
            title: status.to_string(),
            detail,
        }
    }
}

impl IntoResponse for StatusResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}

fn status_as_u16<S: Serializer>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(status.as_u16())
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::InternalServerError(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                StatusResponse::new(StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
            Self::BadRequest(detail) => {
                StatusResponse::with_detail(StatusCode::BAD_REQUEST, detail).into_response()
            }
            Self::Conflict(detail) => {
                StatusResponse::with_detail(StatusCode::CONFLICT, detail).into_response()
            }
            Self::Unauthorized(detail) => {
                StatusResponse::with_detail(StatusCode::UNAUTHORIZED, detail).into_response()
            }
        }
    }
}

pub fn app() -> axum::Router<Arc<AppState>> {
    routes::build_router()
}
