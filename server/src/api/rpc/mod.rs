mod task_rpc;
use crate::api::rpc::task_rpc::{create_task, delete_task, list_tasks, update_task};
use crate::api::{Error, Result};
use crate::model::task::{TaskCreate, TaskUpdate};
use crate::model::ModelManager;
use async_trait::async_trait;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::value::from_value;
use serde_json::value::to_value;
use serde_json::{json, Value};
use tracing::debug;

/// JSON-RPC Request Body.
#[derive(Debug, Deserialize)]
struct RpcRequest {
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/rpc", post(rpc_handler))
        .with_state(mm)
}

async fn rpc_handler(
    State(mm): State<ModelManager>,
    Json(rpc_req): Json<RpcRequest>,
) -> Result<Json<Value>> {
    let RpcRequest {
        id: rpc_id,
        method: rpc_method,
        params: rpc_params,
    } = rpc_req;

    debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

    let result_json: Value = match rpc_method.as_str() {
        "create_task" => CreateTaskRpc.execute(mm, rpc_params).await?,
        "list_tasks" => ListTasksRpc.execute(mm, rpc_params).await?,
        "updated_task" => UpdateTaskRpc.execute(mm, rpc_params).await?,
        "deleted_task" => DeleteTaskRpc.execute(mm, rpc_params).await?,
        _ => return Err(Error::RpcMethodUnknown(rpc_method)),
    };

    let body_response = json!({
        "id": rpc_id,
        "result": result_json
    });

    Ok(Json(body_response))
}

#[async_trait]
pub trait RpcMethod {
    async fn execute(&self, mm: ModelManager, params: Option<Value>) -> Result<Value>;
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
    data: D,
}

struct CreateTaskRpc;

#[async_trait]
impl RpcMethod for CreateTaskRpc {
    async fn execute(&self, mm: ModelManager, params: Option<Value>) -> Result<Value> {
        let params: ParamsForCreate<TaskCreate> = params
            .ok_or(Error::RpcMissingParams {
                rpc_method: stringify!(CreateTaskRpc).to_string(),
            })
            .and_then(|v| {
                from_value(v).map_err(|_| Error::RpcFailJsonParams {
                    rpc_method: stringify!(CreateTaskRpc).to_string(),
                })
            })?;

        // Invoke your existing `create_task` logic with the deserialized parameters.
        let ParamsForCreate { data } = params;
        create_task(mm, ParamsForCreate { data }).await?;

        Ok(json!({ "success": true }))
    }
}

// Implement the `RpcMethod` for listing tasks.
struct ListTasksRpc;

#[async_trait]
impl RpcMethod for ListTasksRpc {
    async fn execute(&self, mm: ModelManager, _params: Option<Value>) -> Result<Value> {
        // Call the list_tasks function.
        let tasks = list_tasks(mm).await?;

        // Convert the result to a JSON Value.
        let result_json = to_value(tasks)?;

        Ok(result_json)
    }
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
    id: i64,
    data: D,
}

struct UpdateTaskRpc;

#[async_trait]
impl RpcMethod for UpdateTaskRpc {
    async fn execute(&self, mm: ModelManager, params: Option<Value>) -> Result<Value> {
        let params: ParamsForUpdate<TaskUpdate> = params
            .ok_or(Error::RpcMissingParams {
                rpc_method: stringify!(UpdateTaskRpc).to_string(),
            })
            .and_then(|v| {
                from_value(v).map_err(|_| Error::RpcFailJsonParams {
                    rpc_method: stringify!(UpdateTaskRpc).to_string(),
                })
            })?;

        // Extract the id and data from the deserialized params.
        let ParamsForUpdate { id, data } = params;

        // Call the update_task function.
        let updated_task = update_task(mm, ParamsForUpdate { id, data }).await?;

        // Convert the result to a JSON Value.
        let result_json = to_value(updated_task)?;

        Ok(result_json)
    }
}
#[derive(Deserialize)]
pub struct ParamsIded {
    id: i64,
}

// Implement the `RpcMethod` for deleting a task.
struct DeleteTaskRpc;

#[async_trait]
impl RpcMethod for DeleteTaskRpc {
    async fn execute(&self, mm: ModelManager, params: Option<Value>) -> Result<Value> {
        // Deserialize the input parameters.
        let params: ParamsIded = params
            .ok_or(Error::RpcMissingParams {
                rpc_method: stringify!(DeleteTaskRpc).to_string(),
            })
            .and_then(|v| {
                from_value(v).map_err(|_| Error::RpcFailJsonParams {
                    rpc_method: stringify!(DeleteTaskRpc).to_string(),
                })
            })?;

        // Extract the id from the deserialized params.
        let ParamsIded { id } = params;

        // Call the delete_task function.
        let deleted_task = delete_task(mm, ParamsIded { id }).await?;

        // Convert the result to a JSON Value.
        let result_json = to_value(deleted_task)?;

        Ok(result_json)
    }
}
