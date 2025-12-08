use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::user::User;
use serde_json::{Value, json};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::apierror::ApiError;

#[instrument(skip(state))]
pub async fn post_handler(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let mut a = state.pool.acquire().await.unwrap();

    let user = User::new();

    match repositories::user::persist_user(&mut a, &user).await {
        Ok(_) => Ok(Json::from(json!({"id": user.get_id()}))),
        Err(_) => {
            debug!("error");
            Err(ApiError::Error)
        }
    }
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut a = state.pool.acquire().await.unwrap();

    let user = repositories::user::get_user(&mut a, &id).await?;

    match repositories::user::delete_user(&mut a, &user).await {
        Ok(_) => Ok(Json::from(json!("delete ok"))),
        Err(_) => {
            debug!("error");
            Err(ApiError::OperationFailed)
        }
    }
}
