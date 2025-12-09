use axum::response::{IntoResponse, Response};
use model::repository::UserRepositoryError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("")]
    UserNotFound,

    #[error("")]
    Error,

    #[error("")]
    OperationFailed,
}

impl From<UserRepositoryError> for ApiError {
    fn from(value: UserRepositoryError) -> Self {
        match value {
            UserRepositoryError::DatabaseError => ApiError::Error,
            UserRepositoryError::UserError => ApiError::UserNotFound,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = match self {
            ApiError::Error => json!("some error in the user repository"),
            ApiError::UserNotFound => json!({"type": "UserNotFound"}),
            ApiError::OperationFailed => json!({"type": "OperationFailed"}),
        };

        Response::new(body.to_string().into())
    }
}
