mod error;

pub use self::error::{Error, Result};
use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub type Db = Pool<Postgres>;

pub(crate) async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&config::config().DB_URL)
        .await
        .map_err(|e| Error::FailToCreatePool(e.to_string()))
}
