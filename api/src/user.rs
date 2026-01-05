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
pub async fn create_user(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let result = match business::user::new_user(state.pool).await {
        Ok(r) => json!(r),
        Err(r) => json!(r.to_string()),
    };

    Ok(Json::from(result))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let result = match business::user::delete_user(state.pool, id).await {
        Ok(_) => "bien".to_string(),
        Err(r) => r.to_string(),
    };

    Ok(Json::from(json!(result)))
}
