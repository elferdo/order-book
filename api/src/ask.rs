use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    pub price: f32,
}

#[instrument(skip(state))]
pub async fn post_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<AskRequest>,
) -> Result<Json<Value>, ApiError> {
    let result = business::ask::new_ask(state.pool, user_id, body.price).await?;

    Ok(Json::from(json!(result)))
}
