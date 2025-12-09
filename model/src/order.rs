use uuid::Uuid;

#[derive(Debug)]
pub enum Order {
    Bid { id: Uuid, user_id: Uuid, price: f32 },
    Ask { id: Uuid, user_id: Uuid, price: f32 },
}

impl Order {
    pub fn new_ask(user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v4();

        Self::Ask { id, user_id, price }
    }

    pub fn new_bid(user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v4();

        Self::Bid { id, user_id, price }
    }

    pub fn ask_with(id: Uuid, user_id: Uuid, price: f32) -> Self {
        Self::Ask { id, user_id, price }
    }

    pub fn bid_with(id: Uuid, user_id: Uuid, price: f32) -> Self {
        Self::Bid { id, user_id, price }
    }

    pub fn get_id(&self) -> &Uuid {
        match self {
            Self::Ask { id, .. } => id,
            Self::Bid { id, .. } => id,
        }
    }

    pub fn get_user_id(&self) -> &Uuid {
        match self {
            Self::Ask { user_id, .. } => user_id,
            Self::Bid { user_id, .. } => user_id,
        }
    }

    pub fn get_price(&self) -> f32 {
        match self {
            Self::Ask { price, .. } => *price,
            Self::Bid { price, .. } => *price,
        }
    }
}
