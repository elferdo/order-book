use thiserror::Error;
use uuid::Uuid;

use super::ask::Ask;
use super::bid::Bid;
use super::order::Order;
use crate::lock_mode::LockMode;

pub trait OrderRepository {
    fn find_asks_below(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> impl Future<Output = Result<Vec<Ask>, OrderRepositoryError>>;

    fn find_bids_above(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> impl Future<Output = Result<Vec<Bid>, OrderRepositoryError>>;

    fn find_ask(&mut self, id: &Uuid) -> impl Future<Output = Result<Order, OrderRepositoryError>>;
    fn find_bid(&mut self, id: &Uuid) -> impl Future<Output = Result<Order, OrderRepositoryError>>;

    fn persist_ask(&mut self, ask: &Ask) -> impl Future<Output = Result<(), OrderRepositoryError>>;
    fn persist_bid(&mut self, bid: &Bid) -> impl Future<Output = Result<(), OrderRepositoryError>>;
}

#[derive(Debug, Error)]
pub enum OrderRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}
