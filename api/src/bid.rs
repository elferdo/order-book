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
    let urepo = repositories::user::Repository::new(state.pool.clone());

    let user = urepo.get_user(&body.user).await?;

    let bid = Bid::new(Arc::new(user), body.price);

    let brepo = repositories::bid::Repository::new(state.pool.clone());

    match brepo.persist_bid(&bid).await {
        Ok(_) => Ok(Json::from(json!({"id": bid.get_id()}))),
        Err(_) => {
            debug!("error");
            Err(ApiError::Error)
        }
    }
}
