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

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use rstest::rstest;
    use uuid::{ContextV7, Uuid};

    #[derive(Default)]
    struct RepositoryMock {
        pub persisted_candidates: Vec<Candidate>,
    }

    impl OrderRepository for RepositoryMock {
        async fn find_asks_below(
            &mut self,
            lock_mode: LockMode,
            bid: &Bid,
        ) -> std::result::Result<Vec<Ask>, RepositoryError> {
            todo!()
        }

        async fn find_bids_above(
            &mut self,
            lock_mode: LockMode,
            ask: &Ask,
        ) -> std::result::Result<Vec<Bid>, RepositoryError> {
            let context = ContextV7::new();
            let timestamp = Timestamp::now(context);

            let user_id = Uuid::new_v7(timestamp);

            let b1 = Bid::new(timestamp, user_id, 2.34);

            let v = vec![b1];

            Ok(v)
        }

        async fn remove_ask(&mut self, ask: &Ask) -> std::result::Result<(), RepositoryError> {
            todo!()
        }

        async fn remove_bid(&mut self, bid: &Bid) -> std::result::Result<(), RepositoryError> {
            todo!()
        }
    }

    impl CandidateRepository for RepositoryMock {
        async fn find_candidate(
            &mut self,
            lock_mode: LockMode,
            id: &uuid::Uuid,
        ) -> std::result::Result<Candidate, RepositoryError> {
            todo!()
        }

        async fn find_candidates_by_user(
            &mut self,
            user: &crate::user::user::User,
        ) -> std::result::Result<Vec<Candidate>, RepositoryError> {
            todo!()
        }

        async fn persist_candidate(
            &mut self,
            candidate: &Candidate,
        ) -> std::result::Result<(), RepositoryError> {
            todo!()
        }

        async fn persist_candidates<I>(
            &mut self,
            iterator: I,
        ) -> std::result::Result<(), RepositoryError>
        where
            I: IntoIterator<Item = Candidate>,
        {
            for candidate in iterator {
                self.persisted_candidates.push(candidate);
            }

            Ok(())
        }

        async fn remove_candidate(
            &mut self,
            candidate: &Candidate,
        ) -> std::result::Result<(), RepositoryError> {
            todo!()
        }
    }

    #[tokio::test]
    async fn given_one_ask_and_one_matching_bid_then_one_candidate() -> Result<()> {
        let mut repo = RepositoryMock::default();

        let context = ContextV7::new();
        let timestamp = Timestamp::now(context);

        let ask = Ask::new(timestamp, Uuid::new_v7(timestamp), 1.23);

        generate_candidates_for_ask(timestamp, &mut repo, &ask).await?;

        assert_eq!(repo.persisted_candidates.len(), 1);

        let candidate = &repo.persisted_candidates[0];

        assert_eq!(*candidate.get_ask(), ask);
        assert_eq!(candidate.get_bid().get_price(), 2.34);

        Ok(())
    }
}
