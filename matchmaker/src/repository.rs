use error_stack::Report;
use order::{
    repository_error::RepositoryError,
    {ask::Ask, bid::Bid},
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
