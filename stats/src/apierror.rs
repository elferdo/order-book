/// This module is where responses from the API are generated
///
/// Here we can control what gets returned to the caller so that
/// no unnecessary details are leaked.
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::businesserror::BusinessError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    #[error("user not found in the database")]
    UserNotFound,

    #[error("database error")]
    DatabaseError,

    #[error("business logic error")]
    BusinessError(#[from] BusinessError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({"type": format!("{self}")});

        Response::new(body.to_string().into())
    }
}
