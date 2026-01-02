use error_stack::Report;
use uuid::Uuid;

use crate::{order::candidate::Candidate, repository_error::RepositoryError, user::user::User};

pub trait CandidateRepository {
    fn find_candidate(
        &mut self,
        id: &Uuid,
    ) -> impl Future<Output = Result<Candidate, Report<RepositoryError>>>;

    fn find_candidates_by_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<Vec<Candidate>, Report<RepositoryError>>>;

    fn persist_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>;

    fn archive_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>;

    fn persist_candidates<I>(
        &mut self,
        iterator: I,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>
    where
        I: IntoIterator<Item = Candidate>;

    fn remove_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>;
}
