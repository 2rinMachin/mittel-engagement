use std::sync::Arc;
use std::task::{Context, Poll};

use axum::response::IntoResponse;
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use tower::{Layer, Service};

use crate::http::ApiError;

#[derive(Clone)]
pub struct InternalAuthLayer {
    token: Arc<String>,
}

impl InternalAuthLayer {
    pub fn new<T: Into<String>>(token: T) -> Self {
        Self {
            token: Arc::new(token.into()),
        }
    }
}

impl<S> Layer<S> for InternalAuthLayer {
    type Service = InternalAuth<S>;

    fn layer(&self, inner: S) -> Self::Service {
        InternalAuth {
            inner,
            token: self.token.clone(),
        }
    }
}

#[derive(Clone)]
pub struct InternalAuth<S> {
    inner: S,
    token: Arc<String>,
}

impl<S> Service<Request<Body>> for InternalAuth<S>
where
    S: Service<Request<Body>, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
    S::Error: Into<axum::BoxError>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let token = self.token.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let received_token = req
                .headers()
                .get("X-Internal-Token")
                .and_then(|h| h.to_str().ok());

            if received_token.is_some_and(|t| t == *token) {
                inner.call(req).await
            } else {
                Ok(ApiError::Unauthorized(None).into_response())
            }
        })
    }
}
