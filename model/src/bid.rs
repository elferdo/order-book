use std::sync::Arc;

use uuid::Uuid;

use crate::user::User;

pub struct Bid {
    id: Uuid,
    user: Arc<User>,
    price: f32,
}

impl Bid {
    pub fn new(user: Arc<User>, price: f32) -> Self {
        let id = Uuid::new_v4();

        Self { id, user, price }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }
}
