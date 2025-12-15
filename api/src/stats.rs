use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{Json, extract::State};
use model::stats::repository::StatsRepository;
use repositories::Repository;
use serde_json::{Value, json};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn buy_price_get_handler(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let buy_price = repo.buy_price().await.map_err(|_| ApiError::UserNotFound)?;

    Ok(Json::from(json!({"buy_price": buy_price})))
}

#[instrument(skip(state))]
pub async fn sell_price_get_handler(
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let sell_price = repo
        .sell_price()
        .await
        .map_err(|_| ApiError::UserNotFound)?;

    Ok(Json::from(json!({"sell_price": sell_price})))
}
