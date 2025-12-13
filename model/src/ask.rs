use std::cmp::Ordering;

use uuid::{Timestamp, Uuid};

#[derive(Debug, PartialEq)]
pub struct Ask {
    id: Uuid,
    user_id: Uuid,
    price: f32,
}

impl Ask {
    pub fn new(t: Timestamp, user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v7(t);

        Self { id, user_id, price }
    }

    pub fn with(id: Uuid, user_id: Uuid, price: f32) -> Self {
        Self { id, user_id, price }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }
}

impl PartialOrd for Ask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
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
}

impl Eq for Ask {}

impl Ord for Ask {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(c) = self.partial_cmp(other) {
            c
        } else {
            // If we can't establish a priority, let's just give both orders equal priority
            Ordering::Equal
        }
    }
}
