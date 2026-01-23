#[cfg(test)]
mod tests;

use error_stack::{Report, ResultExt};
use std::collections::BTreeSet;
use tracing::{debug, error, info, instrument};
use uuid::Timestamp;

use crate::{
    deal::{Deal, repository::DealRepository},
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
) -> Result<(), Report<RepositoryError>>
where
    R: OrderRepository + CandidateRepository,
{
    info!("entering");

    let mut matching_orders: Vec<_> = repository
        .find_bids_not_below(ask)
        .await?
        .into_iter()
        .collect();

    info!("collected matching orders");

    if matching_orders.is_empty() {
        return Ok(());
    };

    let first = matching_orders.pop().expect("this should never fail");

    let candidate = Candidate::new(timestamp, *ask, first);

    repository
        .persist_candidates([candidate])
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

    info!("processing matching orders for ask");

    Ok(())
}

#[instrument(skip(repository))]
pub async fn generate_candidates_for_bid<R>(
    timestamp: Timestamp,
    repository: &mut R,
    bid: &Bid,
) -> Result<(), Report<RepositoryError>>
where
    R: OrderRepository + CandidateRepository,
{
    let mut matching_orders: Vec<_> = repository
        .find_asks_not_above(bid)
        .await?
        .into_iter()
        .collect();

    if matching_orders.is_empty() {
        return Ok(());
    }

    let first = matching_orders.pop().expect("this should never fail");

    let candidate = Candidate::new(timestamp, first, *bid);

    repository
        .persist_candidates([candidate])
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

    info!("processing matching orders for bid");

    Ok(())
}

pub async fn seal<R>(
    repo: &mut R,
    timestamp: Timestamp,
    candidate: Candidate,
) -> Result<Deal, Report<MatchServiceError>>
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

    repo.remove_candidate(&candidate)
        .await
        .change_context(MatchServiceError::Error)?;

    repo.remove_ask(candidate.get_ask())
        .await
        .change_context(MatchServiceError::Error)?;

    repo.remove_bid(candidate.get_bid())
        .await
        .change_context(MatchServiceError::Error)?;

    Ok(deal)
}

pub async fn reject<R>(repo: &mut R, candidate: Candidate) -> Result<(), MatchServiceError>
where
    R: CandidateRepository + DealRepository + OrderRepository,
{
    repo.archive_candidate(&candidate)
        .await
        .map_err(|_| MatchServiceError::Error)?;

    repo.remove_candidate(&candidate)
        .await
        .map_err(|_| MatchServiceError::Error)?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum MatchServiceError {
    #[error("Some error")]
    Error,
}
