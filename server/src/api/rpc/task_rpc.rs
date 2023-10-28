use crate::api::rpc::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::api::Result;
use crate::model::task::{Task, TaskBackendManagerController, TaskCreate, TaskUpdate};
use crate::model::ModelManager;

pub async fn create_task(mm: ModelManager, params: ParamsForCreate<TaskCreate>) -> Result<Task> {
    let ParamsForCreate { data } = params;

    let id = TaskBackendManagerController::create(&mm, data).await?;
    let task = TaskBackendManagerController::get(&mm, id).await?;

    Ok(task)
}

pub async fn list_tasks(mm: ModelManager) -> Result<Vec<Task>> {
    let tasks = TaskBackendManagerController::list(&mm).await?;

    Ok(tasks)
}

pub async fn update_task(mm: ModelManager, params: ParamsForUpdate<TaskUpdate>) -> Result<Task> {
    let ParamsForUpdate { id, data } = params;

    TaskBackendManagerController::update(&mm, id, data).await?;

    let task = TaskBackendManagerController::get(&mm, id).await?;

    Ok(task)
}

pub async fn delete_task(mm: ModelManager, params: ParamsIded) -> Result<Task> {
    let ParamsIded { id } = params;

    let task = TaskBackendManagerController::get(&mm, id).await?;
    TaskBackendManagerController::delete(&mm, id).await?;

    Ok(task)
}
