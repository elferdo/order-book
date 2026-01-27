use crate::repository::MarketRepository;

fn persist() {
    repo.persist_candidates(candidates)
        .await
        .change_context(MarketError::CandidatePersistanceError)?;
}
