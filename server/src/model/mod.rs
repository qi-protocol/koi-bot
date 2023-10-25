#![allow(unused)]
//! Model Layer
//! - the Model layer normalizes the application's data type structures and access
//! - all application's data access must go thru the Model layer
//! - the `ModelManager` holds the internal state/resource needed by the `ModelController` to
//! access data
//! - `ModelController`('ProjectBMC') implements CRUD and other data access methods on a given entity (i.e, `Task`)
//!     - `BMC` stands for `Backend Model Controller`
//! - `ModelManager` is designed to be passed as an argument to all ModelControllers functions
mod base;
mod error;
mod store;
pub mod task;
use self::error::{Error, Result};

#[derive(Clone, Debug)]
pub struct ModelManager {
    db: store::Db,
}

impl ModelManager {
    pub(crate) async fn new() -> Result<Self> {
        let db = store::new_db_pool().await?;
        Ok(Self { db })
    }

    /// Returns a reference of the sqlx db pool
    /// Only within the model layer
    pub(in crate::model) fn db(&self) -> &store::Db {
        &self.db
    }
}
