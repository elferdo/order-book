use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::{bid::Bid, match_maker};
use repositories::{BidRepository, OrderMatchRepository};
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
    let mut t = state.pool.begin().await.unwrap();

    // let mut a = state.pool.acquire().await.unwrap();

    let bid = Bid::new(user_id, body.price);

    // OrderMatchRepository::find_matches_for_bid(&bid).await;

    match BidRepository::persist_bid(&mut t, &bid).await {
        Ok(_) => Ok(Json::from(json!({"id": bid.get_id()}))),
        Err(e) => match e {
            repositories::bid::RepositoryError::DatabaseError(_) => Err(ApiError::Error),
            repositories::bid::RepositoryError::UserError(_) => Err(ApiError::UserNotFound),
        },
    }
}
