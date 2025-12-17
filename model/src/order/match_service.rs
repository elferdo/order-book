use crate::order::{
    candidate::Candidate, candidate_repository::CandidateRepository, repository::OrderRepository,
};

pub async fn commit_candidate<R>(
    repo: &mut R,
    candidate: Candidate,
) -> Result<(), MatchServiceError>
where
    R: CandidateRepository + OrderRepository,
{
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
