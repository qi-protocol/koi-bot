pub mod _dev_utils;
mod config;
mod ctx;
mod error;
mod model;

pub use config::config;
pub use error::{Error, Result};

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .without_time() // For dev enb
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    _dev_utils::init_dev().await;

    let routes_all = Router::new().merge(routes_hello());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let _ = axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await;
}

fn routes_hello() -> Router {
    let routes = Router::new()
        .route("/hello_param", get(handle_hello_param))
        .route("/hello_path/:name", get(handle_hello_path));
    routes
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handle_hello_param(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, {}!", name))
}

async fn handle_hello_path(Path(params): Path<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, {}!", name))
}
