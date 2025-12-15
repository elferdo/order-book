use thiserror::Error;

use crate::{order_match::Match, user::user::User};

pub trait OrderMatchRepository {
    fn find_order_matches_by_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<Vec<Match>, OrderMatchRepositoryError>>;

    fn persist_order_match(
        &mut self,
        order_match: &Match,
    ) -> impl Future<Output = Result<(), OrderMatchRepositoryError>>;

    fn persist_order_matches<I>(
        &mut self,
        iterator: I,
    ) -> impl Future<Output = Result<(), OrderMatchRepositoryError>>
    where
        I: IntoIterator<Item = Match>;
}

#[derive(Debug, Error)]
pub enum OrderMatchRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}
