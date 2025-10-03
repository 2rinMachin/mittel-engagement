use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::{IntoParams, OpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    db::CreateEventRequest,
    domain::{Device, Event},
    http::{
        ApiError, ApiResult, StatusResponse, extractors::RequestUser,
        middleware::InternalAuthLayer, state::AppState,
    },
};

async fn get_not_found(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    StatusResponse::new(StatusCode::NOT_FOUND)
}

#[utoipa::path(get, path = "/", description = "Returns a status message", responses((status = OK, body = StatusResponse)))]
async fn get_hello(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
    StatusResponse::with_detail(StatusCode::OK, Some("It works!".to_owned()))
}

#[utoipa::path(get, path = "/events", params(EventQuery), description = "Returns all events", responses((status = OK, body = [Event])))]
async fn get_events(
    Query(query): Query<EventQuery>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<Event>>> {
    let events = state
        .repo
        .find_events(&query.user_id, &query.post_id)
        .await?;
    Ok(Json(events))
}

#[utoipa::path(get, path = "/devices", params(EventQuery), description = "Returns all recorded devices", responses((status = OK, body = [Device])))]
async fn get_devices(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<Device>>> {
    let events = state.repo.find_devices().await?;
    Ok(Json(events))
}

#[derive(Debug, Clone, IntoParams, Deserialize)]
#[into_params(parameter_in = Query)]
struct EventQuery {
    post_id: Option<String>,
    user_id: Option<String>,
}

#[utoipa::path(post, path = "/events", description = "Records a new event", responses((status = CREATED, body = StatusResponse)))]
async fn create_event(
    State(state): State<Arc<AppState>>,
    user: Option<RequestUser>,
    Json(event): Json<CreateEventRequest>,
) -> ApiResult<StatusResponse> {
    if !state.posts.validate_post_id(&event.post_id).await? {
        return Err(ApiError::BadRequest(Some("Invalid post ID".to_owned())));
    }

    let user_id = user.map(|RequestUser(user)| user.id);

    state.repo.create_event(event, user_id).await?;
    Ok(StatusResponse::new(StatusCode::CREATED))
}

#[derive(OpenApi)]
#[openapi(info(
    title = "Mittel Engagement",
    description = "API for managing engagement on the Mittel blogging platform.",
    version = "0.1.0",
))]
struct ApiDoc;

pub fn build_router(secret_token: &str) -> axum::Router<Arc<AppState>> {
    let private_router = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_events))
        .routes(routes!(get_devices))
        .route_layer(InternalAuthLayer::new(secret_token));

    let public_router = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(get_hello))
        .routes(routes!(create_event));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        // Private endpoints
        .merge(private_router)
        .merge(public_router)
        .fallback(get_not_found)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
        .split_for_parts();

    let swagger = SwaggerUi::new("/docs").url("/openapi.json", api);
    router.merge(swagger)
}
