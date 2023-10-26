use crate::ctx::Ctx;
use crate::model::{shared_trait, Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;

#[derive(Debug, Fields, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize, Fields)]
pub struct TaskCreate {
    pub title: String,
}

#[derive(Deserialize, Fields)]
pub struct TaskUpdate {
    pub title: Option<String>,
}

pub struct TaskBackendManagerController {}

impl shared_trait::DatabaseController for TaskBackendManagerController {
    const TABLE: &'static str = "take";
}

impl TaskBackendManagerController {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, task_c: TaskCreate) -> Result<i64> {
        shared_trait::create::<Self, _>(ctx, mm, task_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        shared_trait::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        shared_trait::list::<Self, _>(ctx, mm).await
    }
}

#[cfg(test)]
mod tests {

    #![allow(unused)]
    use super::*;
    use crate::_dev_utils;
    use crate::model::Error;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // Setup & Fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        // Exec
        let task_c = TaskCreate {
            title: fx_title.to_string(),
        };
        let id = TaskBackendManagerController::create(&ctx, &mm, task_c).await?;

        // Check
        let (title,): (String,) = sqlx::query_as("SELECT title from task where id = $1")
            .bind(id)
            .fetch_one(mm.db())
            .await?;
        assert_eq!(title, fx_title);

        // Clean
        let del_count = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(mm.db())
            .await?
            .rows_affected();
        assert_eq!(del_count, 1, "Delete count is not 1");

        Ok(())
    }
}
