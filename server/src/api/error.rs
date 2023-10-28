use crate::model;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use serde_json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    /// Login - Username not found
    UserNameNotFound,
    /// RPC - Unknon method
    RpcMethodUnknown(String),
    /// RPC - Missing params
    RpcMissingParams { rpc_method: String },
    /// RPC - Invalid json arams
    RpcFailJsonParams { rpc_method: String },
    /// Modules
    Model(model::Error),
    /// SerdeJson Error
    SerdeJson(String),
}

impl From<model::Error> for Error {
    fn from(val: model::Error) -> Self {
        Error::Model(val)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::SerdeJson(val.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        log::debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(self);

        response
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl Error {
    #[allow(dead_code)]
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        use crate::api::Error::*;

        #[allow(unreachable_patterns)]
        match self {
            Model(model::Error::EntityNotFound { entity, id }) => (
                StatusCode::BAD_REQUEST,
                ClientError::ENTITY_NOT_FOUND { entity, id: *id },
            ),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types, dead_code)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    ENTITY_NOT_FOUND { entity: &'static str, id: i64 },

    SERVICE_ERROR,
}
// endregion: --- Client Error
