use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("user not found in the database")]
    UserNotFound,

    #[error("general error, no further details")]
    Error,

    #[error("database error")]
    DatabaseError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({"type": format!("{self}")});

        Response::new(body.to_string().into())
    }
}
