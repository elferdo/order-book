use std::sync::Arc;

use uuid::{Timestamp, Uuid};

use crate::{ask::Ask, bid::Bid};

#[derive(Debug)]
pub struct Match {
    id: Uuid,
    ask: Arc<Ask>,
    bid: Arc<Bid>,
}

impl Match {
    pub fn new(t: Timestamp, ask: Arc<Ask>, bid: Arc<Bid>) -> Self {
        let id = Uuid::new_v7(t);

        Self { id, ask, bid }
    }

    pub fn with(id: Uuid, ask: Arc<Ask>, bid: Arc<Bid>) -> Self {
        Self { id, ask, bid }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_ask(&self) -> &Ask {
        &self.ask
    }

    pub fn get_bid(&self) -> &Bid {
        &self.bid
    }

    pub fn get_price(&self) -> f32 {
        (self.ask.get_price() + self.bid.get_price()) / 2.0
    }
}
