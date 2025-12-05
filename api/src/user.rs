use std::sync::Arc;

use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::user::User;
use repositories::{bid, user};
use serde_json::{Value, json};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::apierror::ApiError;

#[instrument(skip(state))]
pub async fn post_handler(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let t = state.pool.begin().await.map_err(|_| ApiError::Error)?;

    let shared_t = Arc::new(t);

    let user = User::new();

    // let mut urepo = repositories::user::Repository::new(shared_t);

    match repositories::user::persist_user(shared_t, &user).await {
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
    let t = state.pool.begin().await.map_err(|_| ApiError::Error)?;

    let shared_t = Arc::new(t);

    // let mut urepo = repositories::user::Repository::new(shared_t);

    let user = repositories::user::get_user(shared_t.clone(), &id).await?;

    match repositories::user::delete_user(shared_t.clone(), &user).await {
        Ok(_) => Ok(Json::from(json!("delete ok"))),
        Err(_) => {
            debug!("error");
            Err(ApiError::OperationFailed)
        }
    }
}
