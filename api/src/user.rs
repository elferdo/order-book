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
pub async fn post_handler(State(state): State<AppState>) -> Result<Json<Value>, Json<Value>> {
    let user = User::new();

    let urepo = repositories::user::Repository::new(state.pool);

    match urepo.persist_user(&user).await {
        Ok(_) => Ok(Json::from(json!({"id": user.get_id()}))),
        Err(_) => {
            debug!("error");
            Err(Json::from(json!("error")))
        }
    }
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let urepo = repositories::user::Repository::new(state.pool);

    let user = urepo.get_user(&id).await?;

    match urepo.delete_user(&user).await {
        Ok(_) => Ok(Json::from(json!("delete ok"))),
        Err(_) => {
            debug!("error");
            Err(ApiError::OperationFailed)
        }
    }
}
