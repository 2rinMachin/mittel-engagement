use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};

use crate::{
    http::{ApiError, state::AppState},
    users::User,
};

#[derive(Debug, Clone)]
pub struct RequestUser(pub User);

impl FromRequestParts<Arc<AppState>> for RequestUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        if let Some(auth) = auth_header {
            let user = state
                .users
                .fetch_user(auth)
                .await?
                .ok_or_else(|| ApiError::Unauthorized(None))?;
            Ok(RequestUser(user))
        } else {
            Err(ApiError::Unauthorized(None))
        }
    }
}

impl OptionalFromRequestParts<Arc<AppState>> for RequestUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Option<Self>, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        if let Some(auth) = auth_header {
            let user = state
                .users
                .fetch_user(auth)
                .await?
                .ok_or_else(|| ApiError::Unauthorized(None))?;
            Ok(Some(RequestUser(user)))
        } else {
            Ok(None)
        }
    }
}
