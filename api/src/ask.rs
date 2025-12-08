use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::ask::Ask;
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

    let ask = Ask::new(user_id, body.price);

    match repositories::ask::persist_ask(&mut a, &ask).await {
        Ok(_) => Ok(Json::from(json!({"id": ask.get_id()}))),
        Err(e) => match e {
            repositories::ask::RepositoryError::DatabaseError(_) => Err(ApiError::Error),
            repositories::ask::RepositoryError::UserError(_) => Err(ApiError::UserNotFound),
        },
    }
}
