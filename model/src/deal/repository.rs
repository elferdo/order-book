use crate::deal::Deal;

pub trait DealRepository {
    fn persist_deal(
        &mut self,
        deal: &Deal,
    ) -> impl Future<Output = Result<(), DealRepositoryError>>;
}

#[derive(Debug, thiserror::Error)]
pub enum DealRepositoryError {
    #[error("")]
    Error,
}
