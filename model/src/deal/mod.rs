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

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_buyer_id(&self) -> &Uuid {
        &self.buyer
    }

    pub fn get_seller_id(&self) -> &Uuid {
        &self.seller
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }
}
