use error_stack::Report;

use super::ask::Ask;
use super::bid::Bid;
use crate::lock_mode::LockMode;
use crate::repository_error::RepositoryError;

pub trait OrderRepository {
    fn find_asks_not_above(
        &mut self,
        lock_mode: LockMode,
        bid: &Bid,
    ) -> impl Future<Output = Result<Vec<Ask>, Report<RepositoryError>>>;

    fn find_bids_not_below(
        &mut self,
        lock_mode: LockMode,
        ask: &Ask,
    ) -> impl Future<Output = Result<Vec<Bid>, Report<RepositoryError>>>;

    fn remove_ask(
        &mut self,
        ask: &Ask,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>;
    fn remove_bid(
        &mut self,
        bid: &Bid,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>;
}
