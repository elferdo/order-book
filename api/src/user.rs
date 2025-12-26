use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

use crate::apierror::ApiError;

#[instrument(skip(state))]
pub async fn post_handler(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let result = business::user::new_user(state.pool).await?;

    Ok(Json::from(json!(result)))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let result = business::user::delete_user(state.pool, id).await?;

    Ok(Json::from(json!(result)))
}
