use crate::ctx::Ctx;
use crate::model::base;
use crate::model::base::DatabaseBackendController;
use crate::model::Error;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskUpdate {
    pub title: Option<String>,
}

pub struct TaskBackendManagerController {}

impl DatabaseBackendController for TaskBackendManagerController {
    const TABLE: &'static str = "take";
}

impl TaskBackendManagerController {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskCreate) -> Result<i64> {
        let db = mm.db();
        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT INTO task (title) VALUES ($1) RETURNING id")
                .bind(task_c.title)
                .fetch_one(db)
                .await?;

        Ok(id)
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        base::get::<Self, _>(ctx, mm, id).await
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
