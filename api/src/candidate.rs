use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

#[instrument(skip(state))]
pub async fn get_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let result = match business::candidate::get_candidates(state.pool, user_id).await {
        Ok(_) => "bien".to_string(),
        Err(r) => r.to_string(),
    };

    Ok(Json::from(json!(result)))
}

#[instrument(skip(state))]
pub async fn approve_post_handler(
    State(state): State<AppState>,
    Path((user_id, candidate_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    let result =
        match business::candidate::approve_candidate(state.pool, user_id, candidate_id).await {
            Ok(_) => "bien".to_string(),
            Err(r) => r.to_string(),
        };

    Ok(Json::from(json!(result)))
}

#[instrument(skip(state))]
pub async fn reject_post_handler(
    State(state): State<AppState>,
    Path((user_id, candidate_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    let result =
        match business::candidate::reject_candidate(state.pool, user_id, candidate_id).await {
            Ok(_) => "bien".to_string(),
            Err(r) => r.to_string(),
        };

    Ok(Json::from(json!(result)))
}
