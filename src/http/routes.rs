use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use serde_json::json;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    domain::Event,
    http::{ApiResult, state::AppState},
};

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

pub fn build_router() -> axum::Router<Arc<AppState>> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_hello))
        .routes(routes!(get_all_events))
        .split_for_parts();

    let swagger = SwaggerUi::new("/docs").url("/openapi.json", api);

    router.merge(swagger)
}
