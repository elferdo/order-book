use error_stack::Report;
use model::{
    order::{ask::Ask, bid::Bid},
    repository_error::RepositoryError,
};

use crate::candidate::Candidate;

pub trait MarketRepository {
    fn get_unbound_asks(
        &mut self,
    ) -> impl Future<Output = Result<Vec<Ask>, Report<RepositoryError>>>;

    fn get_unbound_bids(
        &mut self,
    ) -> impl Future<Output = Result<Vec<Bid>, Report<RepositoryError>>>;

    fn persist_candidates<I>(
        &mut self,
        iterator: I,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>
    where
        I: IntoIterator<Item = Candidate>;
}
