pub mod repository;

use uuid::{Timestamp, Uuid};

pub struct Deal {
    id: Uuid,
    buyer: Uuid,
    seller: Uuid,
    price: f32,
}

impl Deal {
    pub fn new(timestamp: Timestamp, buyer: Uuid, seller: Uuid, price: f32) -> Self {
        let id = Uuid::new_v7(timestamp);

        Self::with(id, buyer, seller, price)
    }

    pub fn with(id: Uuid, buyer: Uuid, seller: Uuid, price: f32) -> Self {
        Deal {
            id,
            buyer,
            seller,
            price,
        }
    }
}
