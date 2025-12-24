use model::match_service::{self, generate_candidates_for_ask, generate_candidates_for_bid};
use model::order::candidate_repository::CandidateRepository;
use model::user::repository::UserRepository;
use model::{
    lock_mode::LockMode, order::candidate::ApprovalResult, repository_error::RepositoryError,
};
use repositories::Repository;
use sqlx::PgPool;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::businesserror::BusinessError;
use crate::response::Response;

#[instrument(skip(pool))]
pub async fn get_candidates(pool: PgPool, user_id: Uuid) -> Result<Response, BusinessError> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let user = repo
        .find_user(LockMode::KeyShare, &user_id)
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    let _candidates = repo
        .find_candidates_by_user(&user)
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    Ok(Response {})
}

#[instrument(skip(pool))]
pub async fn approve_candidate(
    pool: PgPool,
    user_id: Uuid,
    candidate_id: Uuid,
) -> Result<Response, BusinessError> {
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
        .map_err(|e| match e {
            RepositoryError::DatabaseError(_) => BusinessError::DatabaseError,
            RepositoryError::UnexpectedResult => todo!(),
            RepositoryError::RootEntityNotFound => todo!(),
        })?;

    let mut candidate = repo
        .find_candidate(LockMode::KeyShare, &candidate_id)
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    match user
        .approve(&mut candidate)
        .map_err(|_| BusinessError::DatabaseError)?
    {
        ApprovalResult::Partial => {
            repo.persist_candidate(&candidate)
                .await
                .map_err(|_| BusinessError::DatabaseError)?;
        }

        ApprovalResult::Complete => {
            match_service::seal(&mut repo, timestamp, candidate)
                .await
                .map_err(|_| BusinessError::DatabaseError)?;
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
) -> Result<Response, BusinessError> {
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
