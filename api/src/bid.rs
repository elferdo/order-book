use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use model::match_maker::find_matches_for_order;
use model::repository::UserRepository;
use model::{lock_mode::LockMode, repository::OrderRepositoryError};
use repositories::Repository;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BidRequest {
    pub price: f32,
}

#[instrument(skip(state))]
#[debug_handler]
pub async fn post_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<BidRequest>,
) -> Result<Json<Value>, ApiError> {
    let mut t = state.pool.begin().await.unwrap();

    let mut repo = Repository::new(&mut t).await;

    let user = repo.find_user(LockMode::KeyShare, &user_id).await.unwrap();

    let bid = user.bid(body.price);

    find_matches_for_order(&mut repo, &bid).await;

    match repo.persist_order(&bid).await {
        Ok(_) => {
            t.commit().await.unwrap();

            Ok(Json::from(json!({"id": bid.get_id()})))
        }
        Err(e) => match e {
            OrderRepositoryError::DatabaseError => Err(ApiError::Error),
            OrderRepositoryError::UserError => Err(ApiError::UserNotFound),
        },
    }
}
