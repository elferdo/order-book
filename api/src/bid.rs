use std::sync::Arc;

use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{Json, extract::State};
use model::bid::Bid;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BidRequest {
    pub user_id: Uuid,
    pub price: f32,
}

#[instrument(skip(state))]
pub async fn post_handler(
    State(state): State<AppState>,
    Json(body): Json<BidRequest>,
) -> Result<Json<Value>, ApiError> {
    let t = state.pool.begin().await.map_err(|_| ApiError::Error)?;

    let shared_t = Arc::new(t);

    let bid = Bid::new(body.user_id, body.price);

    let result = match repositories::bid::persist_bid(shared_t.clone(), &bid).await {
        Ok(_) => Ok(Json::from(json!({"id": bid.get_id()}))),
        Err(e) => match e {
            repositories::bid::RepositoryError::DatabaseError(error) => Err(ApiError::Error),
            repositories::bid::RepositoryError::UserError(repository_error) => {
                Err(ApiError::UserNotFound)
            }
        },
    };

    result
}
