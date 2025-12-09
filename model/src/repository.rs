use thiserror::Error;
use uuid::Uuid;

use crate::{
    ask::Ask, bid::Bid, lock_mode::LockMode, order::Order, order_match::Match, user::User,
};

pub trait AskRepository {
    fn find_asks_below(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> impl Future<Output = Result<Vec<Order>, OrderRepositoryError>>;

    fn find_ask(&mut self, id: &Uuid) -> impl Future<Output = Result<Order, OrderRepositoryError>>;
}

pub trait BidRepository {
    fn find_bids_above(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> impl Future<Output = Result<Vec<Order>, OrderRepositoryError>>;

    fn find_bid(&mut self, id: &Uuid) -> impl Future<Output = Result<Order, OrderRepositoryError>>;
}

pub trait UserRepository {
    fn find_user(
        &mut self,
        lock_mode: LockMode,
        id: &Uuid,
    ) -> impl Future<Output = Result<User, UserRepositoryError>>;

    fn persist_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<(), UserRepositoryError>>;

    fn delete_user(&mut self, user: &User)
    -> impl Future<Output = Result<(), UserRepositoryError>>;
}

pub trait OrderMatchRepository {
    fn get_order_match(
        &mut self,
        ask: &Uuid,
        bid: &Uuid,
    ) -> impl Future<Output = Result<Match, OrderMatchRepositoryError>>;

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
pub enum OrderRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}

#[derive(Debug, Error)]
pub enum OrderMatchRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}
