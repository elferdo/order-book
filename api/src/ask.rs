use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::repository::AskRepositoryError;
use model::repository::UserRepository;
use model::{lock_mode::LockMode, repository::AskRepository};
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
    let mut t = state.pool.begin().await.unwrap();

    let mut repo = Repository::new(&mut t).await;

    let user = repo.find_user(LockMode::KeyShared, &user_id).await.unwrap();

    let ask = user.ask(body.price);

    match repo.persist_ask(&ask).await {
        Ok(_) => {
            t.commit().await.unwrap();

            Ok(Json::from(json!({"id": ask.get_id()})))
        }
        Err(e) => match e {
            AskRepositoryError::DatabaseError => Err(ApiError::Error),
            AskRepositoryError::UserError => Err(ApiError::UserNotFound),
        },
    }
}
