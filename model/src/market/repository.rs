use error_stack::Report;

use crate::{order::ask::Ask, order::bid::Bid, repository_error::RepositoryError};

pub trait MarketRepository {
    fn get_unbound_asks(
        &mut self,
    ) -> impl Future<Output = Result<Vec<Ask>, Report<RepositoryError>>>;

    fn get_unbound_bids(
        &mut self,
    ) -> impl Future<Output = Result<Vec<Bid>, Report<RepositoryError>>>;
}
