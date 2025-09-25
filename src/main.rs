pub mod db;
pub mod domain;
pub mod http;
pub mod posts;
pub mod users;

use env_logger::Env;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

use crate::{
    db::MySql, http::state::AppState, posts::MockPostsClient, users::UsersMicroserviceClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let mysql = MySql::new(&database_url).await?;

    let users_url = env::var("USERS_URL").expect("USERS_URL not found");
    let users_client = UsersMicroserviceClient::new(&users_url);

    let posts_client = MockPostsClient;

    let state = Arc::new(AppState {
        repo: Arc::new(mysql),
        users: Arc::new(users_client),
        posts: Arc::new(posts_client),
    });

    let secret_token =
        std::env::var("INTERNAL_SECRET_TOKEN").expect("INTERNAL_SECRET_TOKEN not found");
    let app = crate::http::app(&secret_token).with_state(state);

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let addr = format!("{host}:{port}");

    let listener = TcpListener::bind(addr).await?;

    log::info!("Listening at {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
