use thiserror::Error;
use uuid::Uuid;

use crate::{lock_mode::LockMode, order::candidate::Candidate, user::user::User};

pub trait CandidateRepository {
    fn find_candidate(
        &mut self,
        lock_mode: LockMode,
        id: &Uuid,
    ) -> impl Future<Output = Result<Candidate, CandidateRepositoryError>>;

    fn find_candidates_by_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<Vec<Candidate>, CandidateRepositoryError>>;

    fn persist_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> impl Future<Output = Result<(), CandidateRepositoryError>>;

    fn persist_candidates<I>(
        &mut self,
        iterator: I,
    ) -> impl Future<Output = Result<(), CandidateRepositoryError>>
    where
        I: IntoIterator<Item = Candidate>;
}

#[derive(Debug, Error)]
pub enum CandidateRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}
