use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::lock_mode::LockMode;
use model::order::repository::OrderRepository;
use model::user::repository::UserRepository;
use repositories::Repository;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

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

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let ask = user.ask(timestamp, body.price);

    repo.persist_ask(&ask)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    ask.generate_candidates(&mut repo)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    t.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(Json::from(json!({"id": ask.get_id()})))
}
