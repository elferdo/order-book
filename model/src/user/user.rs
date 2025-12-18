use std::collections::HashMap;
use std::fmt::Debug;

use uuid::{Timestamp, Uuid};

use crate::order::{
    ask::Ask,
    bid::Bid,
    candidate::{ApprovalResult, Candidate},
};

#[derive(Debug)]
pub struct User {
    id: Uuid,
    asks: HashMap<Uuid, Ask>,
    bids: HashMap<Uuid, Bid>,
}

impl User {
    pub fn new(t: Timestamp) -> Self {
        let id = Uuid::new_v7(t);

        Self::with(id)
    }

    pub fn with(id: Uuid) -> Self {
        let asks = HashMap::new();
        let bids = HashMap::new();

        Self { id, asks, bids }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn ask(&mut self, t: Timestamp, price: f32) -> Result<Ask, UserError> {
        let ask = Ask::new(t, self.id, price);

        self.asks.insert(*ask.get_id(), ask);

        Ok(ask)
    }

    pub fn bid(&mut self, t: Timestamp, price: f32) -> Result<Bid, UserError> {
        let bid = Bid::new(t, self.id, price);

        self.bids.insert(*bid.get_id(), bid);

        Ok(bid)
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
