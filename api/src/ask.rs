use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::repository::AskRepository;
use model::{ask::Ask, repository::AskRepositoryError};
use repositories::Repository;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    pub price: f32,
}

#[instrument(skip(state))]
pub async fn post_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<AskRequest>,
) -> Result<Json<Value>, ApiError> {
    let mut a = state.pool.acquire().await.unwrap();

    let mut repo = Repository::new(&mut *a).await;

    let ask = Ask::new(user_id, body.price);

    match repo.persist_ask(&ask).await {
        Ok(_) => Ok(Json::from(json!({"id": ask.get_id()}))),
        Err(e) => match e {
            AskRepositoryError::DatabaseError => Err(ApiError::Error),
            AskRepositoryError::UserError => Err(ApiError::UserNotFound),
        },
    }
}
