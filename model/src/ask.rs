use uuid::Uuid;

pub struct Ask {
    id: Uuid,
    user_id: Uuid,
    price: f32,
}

impl Ask {
    pub fn new(user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v4();

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
