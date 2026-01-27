use crate::deal;
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

use crate::{
    apierror::ApiError,
    {get_candidates, new_user},
};

#[instrument(skip(state))]
pub async fn create_user(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let result = match new_user(state.pool).await {
        Ok(r) => json!(r),
        Err(r) => json!(r.to_string()),
    };

    Ok(Json::from(result))
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let result = match crate::delete_user(state.pool, id).await {
        Ok(_) => "bien".to_string(),
        Err(r) => r.to_string(),
    };

    Ok(Json::from(json!(result)))
}

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    pub price: f32,
}

#[instrument(skip(state))]
pub async fn create_ask(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<AskRequest>,
) -> Result<Json<Value>, ApiError> {
    let result = match crate::new_ask(state.pool, user_id, body.price)
        .await
        .change_context(ApiError::DatabaseError)
    {
        Ok(_) => "bien".to_string(),
        Err(r) => r.to_string(),
    };

    Ok(Json::from(json!(result)))
}

#[derive(Debug, Deserialize)]
pub struct BidRequest {
    pub price: f32,
}

#[instrument(skip(state))]
pub async fn create_bid(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<BidRequest>,
) -> Result<Json<Value>, ApiError> {
    let result = match crate::new_bid(state.pool, user_id, body.price)
        .await
        .change_context(ApiError::DatabaseError)
    {
        Ok(_) => "bien".to_string(),
        Err(r) => r.to_string(),
    };

    Ok(Json::from(json!(result)))
}

#[instrument(err, skip(state))]
pub async fn get_candidate(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    get_candidates(state.pool, user_id)
        .await
        .map_err(|_| ApiError::DatabaseError)
        .map(|v| Json::from(json!(v)))
}

#[instrument(skip(state))]
pub async fn get_deal(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    deal::get_deals(state.pool, user_id)
        .await
        .map_err(|_| ApiError::Error)
        .map(|v| Json::from(json!(v)))
}

#[instrument(skip(state))]
pub async fn approve_candidate(
    State(state): State<AppState>,
    Path((user_id, candidate_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    let result = match crate::approve_candidate(state.pool, user_id, candidate_id).await {
        Ok(_) => "bien".to_string(),
        Err(r) => r.to_string(),
    };

    Ok(Json::from(json!(result)))
}

#[instrument(skip(state))]
pub async fn reject_candidate(
    State(state): State<AppState>,
    Path((user_id, candidate_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    let result = match crate::reject_candidate(state.pool, user_id, candidate_id).await {
        Ok(_) => "bien".to_string(),
        Err(r) => r.to_string(),
    };

    Ok(Json::from(json!(result)))
}
