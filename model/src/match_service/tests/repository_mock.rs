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

    async fn archive_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> std::result::Result<(), RepositoryError> {
        todo!()
    }
}
