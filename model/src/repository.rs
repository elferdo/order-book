use thiserror::Error;
use uuid::Uuid;

use crate::{ask::Ask, bid::Bid, lock_mode::LockMode, order_match::Match, user::User};

pub trait AskRepository {
    fn find_asks_below(
        &mut self,
        price: f32,
    ) -> impl Future<Output = Result<Vec<Ask>, AskRepositoryError>>;

    fn find_ask(&mut self, id: &Uuid) -> impl Future<Output = Result<Ask, AskRepositoryError>>;
    fn persist_ask(&mut self, ask: &Ask) -> impl Future<Output = Result<(), AskRepositoryError>>;
}

pub trait BidRepository {
    fn find_bids_below(
        &mut self,
        price: f32,
    ) -> impl Future<Output = Result<Vec<Bid>, BidRepositoryError>>;

    fn find_bid(&mut self, id: &Uuid) -> impl Future<Output = Result<Bid, BidRepositoryError>>;
    fn persist_bid(&mut self, bid: &Bid) -> impl Future<Output = Result<(), BidRepositoryError>>;
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
}

#[derive(Debug, Error)]
pub enum AskRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}

#[derive(Debug, Error)]
pub enum BidRepositoryError {
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
