use uuid::{Timestamp, Uuid};

use super::ask::Ask;
use super::bid::Bid;

#[derive(Debug, PartialEq)]
pub enum Order {
    Bid { id: Uuid, user_id: Uuid, price: f32 },
    Ask { id: Uuid, user_id: Uuid, price: f32 },
}

impl Order {
    pub fn new_ask(t: Timestamp, user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v7(t);

        Self::Ask { id, user_id, price }
    }

    pub fn new_bid(t: Timestamp, user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v7(t);

        Self::Bid { id, user_id, price }
    }

    pub fn ask_with(id: Uuid, user_id: Uuid, price: f32) -> Self {
        Self::Ask { id, user_id, price }
    }

    pub fn bid_with(id: Uuid, user_id: Uuid, price: f32) -> Self {
        Self::Bid { id, user_id, price }
    }

    pub fn get_id(&self) -> &Uuid {
        match self {
            Self::Ask { id, .. } => id,
            Self::Bid { id, .. } => id,
        }
    }

    pub fn get_user_id(&self) -> &Uuid {
        match self {
            Self::Ask { user_id, .. } => user_id,
            Self::Bid { user_id, .. } => user_id,
        }
    }

    pub fn get_price(&self) -> f32 {
        match self {
            Self::Ask { price, .. } => *price,
            Self::Bid { price, .. } => *price,
        }
    }
}

impl From<&Ask> for Order {
    fn from(value: &Ask) -> Self {
        let id = value.get_id();
        let user_id = value.get_user_id();
        let price = value.get_price();

        Self::ask_with(*id, user_id, price)
    }
}

impl From<&Bid> for Order {
    fn from(value: &Bid) -> Self {
        let id = value.get_id();
        let user_id = value.get_user_id();
        let price = value.get_price();

        Self::bid_with(*id, user_id, price)
    }
}
