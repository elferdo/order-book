use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::bid::Bid;
use model::match_maker::find_matches_for_bid;
use repositories::BidRepository;
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

    let bid = Bid::new(user_id, body.price);

    let mut ar = repositories::AskRepository::new(&mut t);

    find_matches_for_bid(&mut ar, &bid).await;

    if let Err(e) = BidRepository::persist_bid(&mut t, &bid).await {
        match e {
            repositories::bid::RepositoryError::DatabaseError(_) => Err(ApiError::Error),
            repositories::bid::RepositoryError::UserError(_) => Err(ApiError::UserNotFound),
        }
    } else {
        Ok(Json::from(json!({"id": bid.get_id()})))
    }
}
