use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::user::User;
use model::{lock_mode::LockMode, repository::UserRepository};
use repositories::Repository;
use serde_json::{Value, json};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::apierror::ApiError;

#[instrument(skip(state))]
pub async fn post_handler(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let mut a = state.pool.acquire().await.unwrap();

    let mut repo = Repository::new(&mut a).await;

    let user = User::new();

    if (repo.persist_user(&user).await).is_ok() {
        Ok(Json::from(json!({"id": user.get_id()})))
    } else {
        debug!("error");
        Err(ApiError::Error)
    }
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut a = state.pool.acquire().await.unwrap();

    let mut repo = Repository::new(&mut a).await;

    let user = repo.find_user(LockMode::None, &id).await.unwrap();

    if repo.delete_user(&user).await.is_ok() {
        Ok(Json::from(json!("delete ok")))
    } else {
        debug!("error");
        Err(ApiError::OperationFailed)
    }
}
