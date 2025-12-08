use uuid::Uuid;

pub struct Match {
    ask: Uuid,
    bid: Uuid,
}

impl Match {
    pub fn new(ask: Uuid, bid: Uuid) -> Self {
        Self { ask, bid }
    }

    pub fn get_ask(&self) -> &Uuid {
        &self.ask
    }

    pub fn get_bid(&self) -> &Uuid {
        &self.bid
    }
}
