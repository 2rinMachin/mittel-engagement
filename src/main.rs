pub mod db;
pub mod domain;
pub mod http;

use env_logger::Env;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

use crate::{db::MySql, http::state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let postgres = MySql::new(&database_url).await?;

    let state = Arc::new(AppState {
        repo: Arc::new(postgres),
    });

    let app = crate::http::app().with_state(state);

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let addr = format!("{host}:{port}");

    let listener = TcpListener::bind(addr).await?;

    log::info!("Listening at {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
