use axum::response::{IntoResponse, Response};
use repositories::user::RepositoryError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("")]
    UserNotFound,

    #[error("")]
    Error,
}

impl From<repositories::user::RepositoryError> for ApiError {
    fn from(value: repositories::user::RepositoryError) -> Self {
        match value {
            RepositoryError::DatabaseError(_) => ApiError::Error,
            RepositoryError::UserNotFound => ApiError::UserNotFound,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = match self {
            ApiError::Error => json!("some error in the user repository"),
            ApiError::UserNotFound => json!({"type": "UserNotFound"}),
        };

        Response::new(body.to_string().into())
    }
}
