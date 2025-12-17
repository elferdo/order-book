use crate::{deal::Deal, user::user::User};

pub trait DealRepository {
    fn persist_deal(
        &mut self,
        deal: &Deal,
    ) -> impl Future<Output = Result<(), DealRepositoryError>>;

    fn find_deals_by_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<Vec<Deal>, DealRepositoryError>>;
}

#[derive(Debug, thiserror::Error)]
pub enum DealRepositoryError {
    #[error("")]
    Error,

    #[error("")]
    DatabaseError,
}
