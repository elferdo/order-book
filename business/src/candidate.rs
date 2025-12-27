use error_stack::Report;
use error_stack::ResultExt;
use model::match_service::{self, generate_candidates_for_ask, generate_candidates_for_bid};
use model::order::candidate::Candidate;
use model::order::candidate_repository::CandidateRepository;
use model::user::repository::UserRepository;
use model::{
    lock_mode::LockMode, order::candidate::ApprovalResult, repository_error::RepositoryError,
};
use repositories::Repository;
use serde::Serialize;
use sqlx::PgPool;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::businesserror::BusinessError;

#[derive(Serialize)]
pub struct CandidateSummary {
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

#[derive(Serialize)]
pub struct Response {}

#[instrument(skip(pool))]
pub async fn get_candidates(
    pool: PgPool,
    user_id: Uuid,
) -> Result<Vec<CandidateSummary>, Report<BusinessError>> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let user = repo
        .find_user(LockMode::KeyShare, &user_id)
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    let candidates = repo
        .find_candidates_by_user(&user)
        .await
        .map_err(|_| BusinessError::DatabaseError)?
        .into_iter()
        .map(CandidateSummary::from)
        .collect();

    Ok(candidates)
}

#[instrument(skip(pool))]
pub async fn approve_candidate(
    pool: PgPool,
    user_id: Uuid,
    candidate_id: Uuid,
) -> Result<Response, Report<BusinessError>> {
    let mut conn = pool
        .begin()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let user = repo
        .find_user(LockMode::KeyShare, &user_id)
        .await
        .change_context(BusinessError::UserNotFound)?;

    let mut candidate = repo
        .find_candidate(LockMode::KeyShare, &candidate_id)
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    match user
        .approve(&mut candidate)
        .change_context(BusinessError::DatabaseError)?
    {
        ApprovalResult::Partial => {
            repo.persist_candidate(&candidate)
                .await
                .map_err(|_| BusinessError::DatabaseError)?;
        }

        ApprovalResult::Complete => {
            match_service::seal(&mut repo, timestamp, candidate)
                .await
                .change_context(BusinessError::DatabaseError)?;
        }
    };

    conn.commit()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    Ok(Response {})
}

#[instrument(skip(pool))]
pub async fn reject_candidate(
    pool: PgPool,
    user_id: Uuid,
    candidate_id: Uuid,
) -> Result<Response, Report<BusinessError>> {
    let mut conn = pool
        .begin()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let candidate = repo
        .find_candidate(LockMode::KeyShare, &candidate_id)
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    match_service::reject(&mut repo, candidate)
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    generate_candidates_for_ask(timestamp, &mut repo, candidate.get_ask())
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    generate_candidates_for_bid(timestamp, &mut repo, candidate.get_bid())
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    conn.commit()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    Ok(Response {})
}
