use uuid::Uuid;

pub struct Bid {
    id: Uuid,
    user_id: Uuid,
    price: f32,
}

impl Bid {
    pub fn new(user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v4();

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
