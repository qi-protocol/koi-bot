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
use axum::Router;
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

    let rpc_route = rpc::routes(model_manager.clone());

    let routes = Router::new().nest("/api", rpc_route);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let _ = axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await;

    Ok(())
}
