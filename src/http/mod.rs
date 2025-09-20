pub mod routes;
pub mod state;

use std::sync::Arc;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

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
}

#[derive(Debug, Clone, Serialize)]
pub struct StatusResponse {
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

pub fn app() -> axum::Router<Arc<AppState>> {
    routes::build_router()
}
