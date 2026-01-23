use std::cmp::Ordering;
use uuid::{Timestamp, Uuid};

use crate::order::score::Score;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bid {
    pub id: Uuid,
    pub buyer: Uuid,
    pub not_above: f32,
}

impl Bid {
    pub fn new(t: Timestamp, user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v7(t);

        Self {
            id,
            buyer: user_id,
            not_above: price,
        }
    }

    pub fn with(id: Uuid, user_id: Uuid, price: f32) -> Self {
        Self {
            id,
            buyer: user_id,
            not_above: price,
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_user_id(&self) -> &Uuid {
        &self.buyer
    }

    pub fn get_price(&self) -> f32 {
        self.not_above
    }

    pub fn sort_fn(one: &Self, other: &Self) -> std::cmp::Ordering {
        if one.not_above > other.not_above {
            Ordering::Less
        } else if one.not_above < other.not_above {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
