use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use error_stack::ResultExt;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BidRequest {
    pub price: f32,
}

#[instrument(skip(state))]
pub async fn post_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<BidRequest>,
) -> Result<Json<Value>, ApiError> {
    let result = match business::bid::new_bid(state.pool, user_id, body.price)
        .await
        .change_context(ApiError::DatabaseError)
    {
        Ok(_) => "bien".to_string(),
        Err(r) => r.to_string(),
    };

    Ok(Json::from(json!(result)))
}
