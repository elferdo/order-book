use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::{lock_mode::LockMode, order::candidate::ApprovalResult};
use model::{order::candidate::Candidate, order::candidate_repository::CandidateRepository};
use model::{order::match_service, user::repository::UserRepository};
use repositories::Repository;
use serde::Serialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize)]
struct CandidateSummary {
    pub id: Uuid,
    pub ask: Uuid,
    pub bid: Uuid,
    pub price: f32,
}

impl From<Candidate> for CandidateSummary {
    fn from(value: Candidate) -> Self {
        let id = *value.get_id();
        let ask = *value.get_ask().get_id();
        let bid = *value.get_bid().get_id();
        let price = value.get_price();

        Self {
            id,
            ask,
            bid,
            price,
        }
    }
}

#[instrument(skip(state))]
pub async fn get_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let user = repo
        .find_user(LockMode::KeyShare, &user_id)
        .await
        .map_err(|_| ApiError::UserNotFound)?;

    let candidates: Vec<CandidateSummary> = repo
        .find_candidates_by_user(&user)
        .await
        .map_err(|_| ApiError::DatabaseError)?
        .into_iter()
        .map(|m| m.into())
        .collect();

    Ok(Json::from(json!(candidates)))
}

pub async fn approve_post_handler(
    State(state): State<AppState>,
    Path((user_id, candidate_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    let mut conn = state
        .pool
        .begin()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut candidate = repo
        .find_candidate(LockMode::KeyShare, &candidate_id)
        .await
        .map_err(|_| ApiError::UserNotFound)?;

    match candidate
        .approve(&user_id)
        .await
        .map_err(|_| ApiError::DatabaseError)?
    {
        ApprovalResult::Partial => {
            repo.persist_candidate(&candidate)
                .await
                .map_err(|_| ApiError::DatabaseError)?;
        }

        ApprovalResult::Complete => {
            match_service::seal(&mut repo, timestamp, candidate)
                .await
                .map_err(|_| ApiError::DatabaseError)?;
        }
    };

    conn.commit().await.map_err(|_| ApiError::DatabaseError)?;

    Ok(Json::from(json!("ok")))
}
