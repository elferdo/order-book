use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::lock_mode::LockMode;
use model::match_maker::find_matches_for_bid;
use model::repository::BidRepository;
use model::repository::UserRepository;
use model::{bid::Bid, repository::BidRepositoryError};
use repositories::Repository;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::Acquire;
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
    let a = t.acquire().await.unwrap();

    let mut repo = Repository::new(&mut *a).await;

    let _user = repo.find_user(LockMode::KeyShared, &user_id).await.unwrap();

    let bid = Bid::new(user_id, body.price);

    find_matches_for_bid(&mut repo, &bid).await;

    if let Err(e) = repo.persist_bid(&bid).await {
        match e {
            BidRepositoryError::DatabaseError => Err(ApiError::Error),
            BidRepositoryError::UserError => Err(ApiError::UserNotFound),
        }
    } else {
        t.commit().await.unwrap();

        Ok(Json::from(json!({"id": bid.get_id()})))
    }
}
