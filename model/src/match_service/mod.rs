#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use tracing::{error, info, instrument};
use uuid::Timestamp;

use crate::{
    deal::{Deal, repository::DealRepository},
    lock_mode::LockMode,
    order::{
        ask::Ask, bid::Bid, candidate::Candidate, candidate_repository::CandidateRepository,
        repository::OrderRepository,
    },
    repository_error::RepositoryError,
};

#[instrument(skip(repository))]
pub async fn generate_candidates_for_ask<R>(
    timestamp: Timestamp,
    repository: &mut R,
    ask: &Ask,
) -> Result<(), RepositoryError>
where
    R: OrderRepository + CandidateRepository,
{
    let mut matching_orders: BTreeSet<_> = repository
        .find_bids_above(LockMode::KeyShare, ask)
        .await?
        .into_iter()
        .collect();

    if matching_orders.is_empty() {
        return Ok(());
    };

    let first = matching_orders.pop_first().expect("this should never fail");

    let candidate = Candidate::new(timestamp, *ask, first);

    if let Err(e) = repository.persist_candidates([candidate]).await {
        match e {
            RepositoryError::DatabaseError(e) => {
                error!("{e}");
            }
            RepositoryError::UnexpectedResult => todo!(),
            RepositoryError::RootEntityNotFound => todo!(),
        }
    };

    info!("processing matching orders for ask");

    Ok(())
}

#[instrument(skip(repository))]
pub async fn generate_candidates_for_bid<R>(
    timestamp: Timestamp,
    repository: &mut R,
    bid: &Bid,
) -> Result<(), RepositoryError>
where
    R: OrderRepository + CandidateRepository,
{
    let mut matching_orders: BTreeSet<_> = repository
        .find_asks_below(LockMode::KeyShare, bid)
        .await?
        .into_iter()
        .collect();

    if matching_orders.is_empty() {
        return Ok(());
    }

    let first = matching_orders.pop_first().expect("this should never fail");

    let candidate = Candidate::new(timestamp, first, *bid);

    if let Err(e) = repository.persist_candidates([candidate]).await {
        match e {
            RepositoryError::DatabaseError(e) => {
                error!("{e}");
            }
            RepositoryError::UnexpectedResult => todo!(),
            RepositoryError::RootEntityNotFound => todo!(),
        }
    };

    info!("processing matching orders for bid");

    Ok(())
}

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

pub async fn reject<R>(repo: &mut R, candidate: Candidate) -> Result<Deal, MatchServiceError>
where
    R: CandidateRepository + DealRepository + OrderRepository,
{
    repo.archive_candidate(&candidate)
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
