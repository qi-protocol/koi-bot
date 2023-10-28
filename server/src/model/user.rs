use crate::ctx::Ctx;
use crate::model::{shared_trait, shared_trait::DatabaseController, Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::{postgres::PgRow, FromRow};

pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send + Sync {}

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForSignup {
    pub id: i64,
    pub username: String,
}

impl UserBy for UserForSignup {}
impl UserBy for User {}

pub struct UserBackendManagerController {}

impl shared_trait::DatabaseController for UserBackendManagerController {
    const TABLE: &'static str = "user";
}

impl UserBackendManagerController {
    /// Gets a user by id
    pub async fn get<E>(mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        shared_trait::get::<Self, _>(mm, id).await
    }

    /// Gets a user by username
    pub async fn first_by_username<E>(mm: &ModelManager, username: &str) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();

        let user = sqlb::select()
            .table(Self::TABLE)
            .and_where("username", "=", username)
            .fetch_optional::<_, E>(db)
            .await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::{Context, Result};
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        // Setup & Fixtures
        let mm = _dev_utils::init_test().await;

        // Inserted in the 02-dev-seed.sql
        let fx_username = "demo1";

        // Exec
        let user: User = UserBackendManagerController::first_by_username(&mm, fx_username)
            .await?
            .context("Should have user 'demo1'")?;

        // Check
        assert_eq!(user.username, fx_username);

        Ok(())
    }
}
