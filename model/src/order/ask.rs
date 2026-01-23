use std::cmp::Ordering;
use uuid::{Timestamp, Uuid};

use crate::order::score::Score;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ask {
    id: Uuid,
    seller: Uuid,
    not_below: f32,
}

impl Ask {
    pub fn new(t: Timestamp, user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v7(t);

        Self {
            id,
            seller: user_id,
            not_below: price,
        }
    }

    pub fn with(id: Uuid, user_id: Uuid, price: f32) -> Self {
        Self {
            id,
            seller: user_id,
            not_below: price,
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_user_id(&self) -> &Uuid {
        &self.seller
    }

    pub fn get_price(&self) -> f32 {
        self.not_below
    }

    pub fn sort_fn(one: &Self, other: &Self) -> std::cmp::Ordering {
        if one.not_below < other.not_below {
            Ordering::Greater
        } else if one.not_below > other.not_below {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
