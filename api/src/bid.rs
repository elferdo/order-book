use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::lock_mode::LockMode;
use model::repository::OrderRepository;
use model::repository::UserRepository;
use repositories::Repository;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

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
    let mut t = state
        .pool
        .begin()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let mut repo = Repository::new(&mut t).await;

    let user = repo
        .find_user(LockMode::KeyShare, &user_id)
        .await
        .map_err(|_| ApiError::UserNotFound)?;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let bid = user.bid(timestamp, body.price);

    repo.persist_bid(&bid)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    bid.generate_matches(&mut repo)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    t.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(Json::from(json!({"id": bid.get_id()})))
}
