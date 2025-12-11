use uuid::{Timestamp, Uuid};

#[derive(Debug)]
pub struct Match {
    id: Uuid,
    ask: Uuid,
    bid: Uuid,
}

impl Match {
    pub fn new(t: Timestamp, ask: Uuid, bid: Uuid) -> Self {
        let id = Uuid::new_v7(t);

        Self { id, ask, bid }
    }

    pub fn with(id: Uuid, ask: Uuid, bid: Uuid) -> Self {
        Self { id, ask, bid }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_ask(&self) -> &Uuid {
        &self.ask
    }

    pub fn get_bid(&self) -> &Uuid {
        &self.bid
    }
}
