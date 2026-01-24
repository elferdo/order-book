use std::collections::HashMap;
use std::fmt::Debug;

use model::order::{
    ask::Ask,
    bid::Bid,
    candidate::{ApprovalResult, Candidate},
};
use uuid::{Timestamp, Uuid};

#[derive(Debug)]
pub struct User {
    id: Uuid,
    asks: HashMap<Uuid, Ask>,
    bids: HashMap<Uuid, Bid>,
}

impl User {
    pub fn new(t: Timestamp) -> Self {
        let id = Uuid::new_v7(t);

        let asks = HashMap::new();
        let bids = HashMap::new();

        Self::with(id, asks, bids)
    }

    pub fn with(id: Uuid, asks: HashMap<Uuid, Ask>, bids: HashMap<Uuid, Bid>) -> Self {
        Self { id, asks, bids }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn ask(&mut self, t: Timestamp, price: f32) -> Ask {
        let ask = Ask::new(t, self.id, price);

        self.asks.insert(*ask.get_id(), ask);

        ask
    }

    pub fn bid(&mut self, t: Timestamp, price: f32) -> Bid {
        let bid = Bid::new(t, self.id, price);

        self.bids.insert(*bid.get_id(), bid);

        bid
    }

    pub fn approve(&self, candidate: &mut Candidate) -> Result<ApprovalResult, UserError> {
        candidate.approve(&self.id).map_err(|_| UserError::Error)
    }

    pub fn asks(&self) -> impl Debug + Iterator<Item = &Ask> {
        self.asks.values()
    }

    pub fn bids(&self) -> impl Debug + Iterator<Item = &Bid> {
        self.bids.values()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("")]
    Error,
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use uuid::{ContextV7, Timestamp};

    #[test]
    fn user_ask() -> Result<()> {
        let context = ContextV7::new();
        let timestamp = Timestamp::now(context);

        let mut user = User::new(timestamp);

        let price = 1.23;

        let _ = user.ask(timestamp, price);

        let asks: Vec<_> = user.asks().collect();

        assert_eq!(asks[0].get_price(), price);

        Ok(())
    }

    #[test]
    fn user_bid() -> Result<()> {
        let context = ContextV7::new();
        let timestamp = Timestamp::now(context);

        let mut user = User::new(timestamp);

        let price = 1.23;

        let _ = user.bid(timestamp, price);

        let bids: Vec<_> = user.bids().collect();

        assert_eq!(bids[0].get_price(), price);

        Ok(())
    }
}
