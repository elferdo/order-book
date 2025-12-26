use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{Json, extract::State};
use serde_json::{Value, json};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn buy_price_get_handler(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let result = business::stats::get_buy_price(state.pool).await?;

    Ok(Json::from(json!(result)))
}

#[instrument(skip(state))]
pub async fn sell_price_get_handler(
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let result = business::stats::get_sell_price(state.pool).await?;

    Ok(Json::from(json!(result)))
}
