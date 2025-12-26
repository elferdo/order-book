use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

#[instrument(skip(state))]
pub async fn get_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let result = business::deal::get_deals(state.pool, user_id).await?;

    Ok(Json::from(json!(result)))
}
