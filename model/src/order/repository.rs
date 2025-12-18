use super::ask::Ask;
use super::bid::Bid;
use crate::lock_mode::LockMode;
use crate::repository_error::RepositoryError;

pub trait OrderRepository {
    fn find_asks_below(
        &mut self,
        lock_mode: LockMode,
        bid: &Bid,
    ) -> impl Future<Output = Result<Vec<Ask>, RepositoryError>>;

    fn find_bids_above(
        &mut self,
        lock_mode: LockMode,
        ask: &Ask,
    ) -> impl Future<Output = Result<Vec<Bid>, RepositoryError>>;

    fn remove_ask(&mut self, ask: &Ask) -> impl Future<Output = Result<(), RepositoryError>>;
    fn remove_bid(&mut self, bid: &Bid) -> impl Future<Output = Result<(), RepositoryError>>;
}
