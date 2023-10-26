pub mod _dev_utils;
mod api;
mod config;
mod ctx;
mod error;
mod model;

pub use config::config;
pub use error::{Error, Result};

use crate::api::rpc;
use crate::model::ModelManager;
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
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // For dev env
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // dev only
    _dev_utils::init_dev().await;

    let model_manager = ModelManager::new().await?;

    let rpc_route =
        rpc::routes::routes(model_manager.clone()).route_layer(middleware::from_fn(mw_ctx));

    let routes_all = Router::new().merge(routes_hello());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let _ = axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await;

    Ok(())
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
