use uuid::Timestamp;

use crate::{
    deal::{Deal, repository::DealRepository},
    order::{
        candidate::Candidate, candidate_repository::CandidateRepository,
        repository::OrderRepository,
    },
};

pub async fn seal<R>(
    repo: &mut R,
    timestamp: Timestamp,
    candidate: Candidate,
) -> Result<Deal, MatchServiceError>
where
    R: CandidateRepository + DealRepository + OrderRepository,
{
    let deal = Deal::new(
        timestamp,
        *candidate.get_buyer_id(),
        *candidate.get_seller_id(),
        candidate.get_price(),
    );

    repo.persist_deal(&deal)
        .await
        .map_err(|_| MatchServiceError::Error)?;

    repo.remove_ask(candidate.get_ask())
        .await
        .map_err(|_| MatchServiceError::Error)?;

    repo.remove_bid(candidate.get_bid())
        .await
        .map_err(|_| MatchServiceError::Error)?;

    repo.remove_candidate(&candidate)
        .await
        .map_err(|_| MatchServiceError::Error)?;

    todo!()
}

#[derive(Debug, thiserror::Error)]
pub enum MatchServiceError {
    #[error("Some error")]
    Error,
}
