use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::repository::UserRepository;
use model::{lock_mode::LockMode, match_maker::find_matches_for_order};
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

    let ask = user.ask(body.price);

    repo.persist_order(&ask)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    find_matches_for_order(&mut repo, &ask).await;

    t.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(Json::from(json!({"id": ask.get_id()})))
}
