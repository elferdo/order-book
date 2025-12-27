use error_stack::Report;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::{
    lock_mode::LockMode,
    order::{
        ask::Ask, bid::Bid, candidate::Candidate, candidate_repository::CandidateRepository,
        repository::OrderRepository,
    },
    repository_error::RepositoryError,
};

#[derive(Default)]
pub(super) struct RepositoryMock {
    pub archived_candidates: Vec<Candidate>,
    pub candidates: Vec<Candidate>,
    pub asks: Vec<Ask>,
    pub bids: Vec<Bid>,
}

impl OrderRepository for RepositoryMock {
    async fn find_asks_not_above(
        &mut self,
        lock_mode: LockMode,
        bid: &Bid,
    ) -> std::result::Result<Vec<Ask>, Report<RepositoryError>> {
        let locked_asks: Vec<_> = self
            .candidates
            .iter()
            .map(|c| c.get_ask())
            .cloned()
            .collect();

        let result = self
            .asks
            .iter()
            .filter(|&ask| ask.get_price() <= bid.get_price() && !locked_asks.contains(&ask))
            .cloned()
            .collect();

        Ok(result)
    }

    async fn find_bids_not_below(
        &mut self,
        lock_mode: LockMode,
        ask: &Ask,
    ) -> std::result::Result<Vec<Bid>, Report<RepositoryError>> {
        let locked_bids: Vec<_> = self
            .candidates
            .iter()
            .map(|c| c.get_bid())
            .cloned()
            .collect();

        let result = self
            .bids
            .iter()
            .filter(|&bid| bid.get_price() >= ask.get_price() && !locked_bids.contains(&bid))
            .cloned()
            .collect();

        Ok(result)
    }

    async fn remove_ask(&mut self, ask: &Ask) -> std::result::Result<(), Report<RepositoryError>> {
        todo!()
    }

    async fn remove_bid(&mut self, bid: &Bid) -> std::result::Result<(), Report<RepositoryError>> {
        todo!()
    }
}

impl CandidateRepository for RepositoryMock {
    async fn find_candidate(
        &mut self,
        lock_mode: LockMode,
        id: &uuid::Uuid,
    ) -> std::result::Result<Candidate, Report<RepositoryError>> {
        let result = self.candidates.iter().find(|&c| c.get_id() == id).cloned();

        result.ok_or(Report::new(RepositoryError::RootEntityNotFound))
    }

    async fn find_candidates_by_user(
        &mut self,
        user: &crate::user::user::User,
    ) -> std::result::Result<Vec<Candidate>, Report<RepositoryError>> {
        todo!()
    }

    async fn persist_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> std::result::Result<(), Report<RepositoryError>> {
        self.candidates.push(*candidate);

        Ok(())
    }

    async fn persist_candidates<I>(
        &mut self,
        iterator: I,
    ) -> std::result::Result<(), Report<RepositoryError>>
    where
        I: IntoIterator<Item = Candidate>,
    {
        for candidate in iterator {
            self.candidates.push(candidate);
        }

        Ok(())
    }

    async fn remove_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> std::result::Result<(), Report<RepositoryError>> {
        if let Some(i) = self
            .candidates
            .iter()
            .position(|c| c.get_id() == candidate.get_id())
        {
            self.candidates.remove(i);
        };

        Ok(())
    }

    async fn archive_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> std::result::Result<(), Report<RepositoryError>> {
        self.archived_candidates.push(*candidate);

        Ok(())
    }
}
