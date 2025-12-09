use uuid::Uuid;

use crate::{ask::Ask, bid::Bid};

pub struct User {
    id: Uuid,
}

impl User {
    pub fn new() -> Self {
        let id = Uuid::new_v4();

        Self { id }
    }

    pub fn new_as(id: Uuid) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn ask(&self, price: f32) -> Ask {
        Ask::new(self.id, price)
    }

    pub fn bid(&self, price: f32) -> Bid {
        Bid::new(self.id, price)
    }
}
