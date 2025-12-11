use std::cmp::{Ordering, PartialOrd};
use uuid::{Timestamp, Uuid};

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

    fn ask_partial_ord_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_price() < other.get_price() {
            Some(Ordering::Less)
        } else if self.get_price() == other.get_price() {
            Some(Ordering::Equal)
        } else if self.get_price() > other.get_price() {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
    fn bid_partial_ord_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_price() < other.get_price() {
            Some(Ordering::Greater)
        } else if self.get_price() == other.get_price() {
            Some(Ordering::Equal)
        } else if self.get_price() > other.get_price() {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Order::Bid { .. } => self.bid_partial_ord_cmp(other),
            Order::Ask { .. } => self.ask_partial_ord_cmp(other),
        }
    }
}

impl Eq for Order {}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(c) = self.partial_cmp(other) {
            c
        } else {
            // If we can't establish a priority, let's just give both orders equal priority
            Ordering::Equal
        }
    }
}
