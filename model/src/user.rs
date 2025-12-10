use uuid::{Timestamp, Uuid};

use crate::order::Order;

pub struct User {
    id: Uuid,
}

impl User {
    pub fn new(t: Timestamp) -> Self {
        let id = Uuid::new_v7(t);

        Self { id }
    }

    pub fn new_as(id: Uuid) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn ask(&self, t: Timestamp, price: f32) -> Order {
        Order::new_ask(t, self.id, price)
    }

    pub fn bid(&self, t: Timestamp, price: f32) -> Order {
        Order::new_bid(t, self.id, price)
    }
}
