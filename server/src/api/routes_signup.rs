use crate::api::error::{Error, Result};
use crate::ctx::Ctx;
use crate::model::user::{UserBackendManagerController, UserForSignup};
use crate::model::ModelManager;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/signup", post(signup_handler))
        .with_state(mm)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
}

pub async fn signup_handler(
    State(mm): State<ModelManager>,
    _cookie: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    log::debug!("{:<12} - api_login_handler", "HANDLER");

    let LoginPayload { username } = payload;
    let root_ctx = Ctx::root_ctx();

    // Get the user.
    // TODO: changed to signup
    let _user: UserForSignup =
        UserBackendManagerController::first_by_username(&root_ctx, &mm, &username)
            .await?
            .ok_or(Error::UserNameNotFound)?;

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
