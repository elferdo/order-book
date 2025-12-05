use std::sync::Arc;

use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{Json, extract::State};
use model::bid::Bid;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::{debug, instrument};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BidRequest {
    pub user: Uuid,
    pub price: f32,
}

#[instrument(skip(state))]
pub async fn post_handler(
    State(state): State<AppState>,
    Json(body): Json<BidRequest>,
) -> Result<Json<Value>, ApiError> {
    let t = state.pool.begin().await.map_err(|_| ApiError::Error)?;

    let shared_t = Arc::new(t);

    let mut urepo = repositories::user::Repository::new(shared_t.clone());

    let user = urepo.get_user(&body.user).await?;

    let bid = Bid::new(Arc::new(user), body.price);

    let mut brepo = repositories::bid::Repository::new(shared_t.clone());

    let result = match brepo.persist_bid(&bid).await {
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
