use error_stack::{IntoReport, Report, ResultExt};
use model::order::candidate::{ApprovalResult, Candidate};
use model::order::candidate_repository::CandidateRepository;
use repositories::Repository;
use serde::Serialize;
use sqlx::{PgPool, query};
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::businesserror::BusinessError;
use crate::repository::UserRepository;
use crate::user::User;

#[derive(Serialize)]
pub struct Response {
    id: Uuid,
}

#[instrument(skip(pool))]
pub async fn new_user(pool: PgPool) -> Result<Response, Report<BusinessError>> {
    let mut a = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    // let mut repo = Repository::new(&mut a).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let user = User::new(timestamp);

    (*a).persist_user(&user)
        .await
        .change_context(BusinessError::DatabaseError)?;

    Ok(Response { id: *user.get_id() })
}

#[instrument(skip(pool))]
pub async fn delete_user(pool: PgPool, id: Uuid) -> Result<Response, Report<BusinessError>> {
    let mut a = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    // let mut repo = Repository::new(&mut a).await;

    let user = (*a)
        .find_user(&id)
        .await
        .change_context(BusinessError::DatabaseError)?;

    (*a).delete_user(&user)
        .await
        .change_context(BusinessError::DatabaseError)?;

    Ok(Response { id: *user.get_id() })
}

#[instrument(skip(pool))]
pub async fn new_ask(
    pool: PgPool,
    user_id: Uuid,
    price: f32,
) -> Result<Response, Report<BusinessError>> {
    let mut t = pool
        .begin()
        .await
        .change_context(BusinessError::DatabaseError)?;

    let mut user = (*t)
        .find_user(&user_id)
        .await
        .change_context(BusinessError::UserNotFound)?;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let ask = user.ask(timestamp, price);

    (*t).persist_user(&user)
        .await
        .change_context(BusinessError::UserPersistenceError)?;

    t.commit()
        .await
        .change_context(BusinessError::DatabaseError)?;

    Ok(Response { id: *ask.get_id() })
}

#[instrument(skip(pool))]
pub async fn new_bid(
    pool: PgPool,
    user_id: Uuid,
    price: f32,
) -> Result<Response, Report<BusinessError>> {
    let mut t = pool
        .begin()
        .await
        .change_context(BusinessError::DatabaseError)?;

    let mut user = (*t)
        .find_user(&user_id)
        .await
        .change_context(BusinessError::UserNotFound)?;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let bid = user.bid(timestamp, price);

    (*t).persist_user(&user)
        .await
        .change_context(BusinessError::UserPersistenceError)?;

    t.commit()
        .await
        .change_context(BusinessError::DatabaseError)?;

    Ok(Response { id: *bid.get_id() })
}

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
pub struct CandidateResponse {}

#[instrument(err, skip(pool))]
pub async fn get_candidates(
    pool: PgPool,
    user_id: Uuid,
) -> Result<Vec<CandidateSummary>, Report<BusinessError>> {
    let mut conn = pool
        .acquire()
        .await
        .change_context(BusinessError::DatabaseError)?;

    let user = (*conn)
        .find_user(&user_id)
        .await
        .change_context(BusinessError::UserNotFound)?;

    let candidates = (*conn)
        .find_candidates_by_user(&user)
        .await
        .change_context(BusinessError::DatabaseError)?
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
) -> Result<CandidateResponse, Report<BusinessError>> {
    let mut conn = pool
        .begin()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let user = repo
        .find_user(&user_id)
        .await
        .change_context(BusinessError::UserNotFound)?;

    let mut candidate = repo
        .find_candidate(&candidate_id)
        .await
        .change_context(BusinessError::CandidateNotFound)?;

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

    Ok(CandidateResponse {})
}

#[instrument(err, skip(pool))]
pub async fn reject_candidate(
    pool: PgPool,
    user_id: Uuid,
    candidate_id: Uuid,
) -> Result<CandidateResponse, Report<BusinessError>> {
    let mut conn = pool
        .begin()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let candidate = repo
        .find_candidate(&candidate_id)
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

    Ok(CandidateResponse {})
}
