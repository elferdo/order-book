use uuid::{Timestamp, Uuid};

use crate::order::{
    ask::Ask,
    bid::Bid,
    candidate::{ApprovalResult, Candidate},
};

#[derive(Debug)]
pub struct User {
    id: Uuid,
}

impl User {
    pub fn new(t: Timestamp) -> Self {
        let id = Uuid::new_v7(t);

        Self { id }
    }

    pub fn new_as(id: Uuid) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn ask(&self, t: Timestamp, price: f32) -> Ask {
        Ask::new(t, self.id, price)
    }

    pub fn bid(&self, t: Timestamp, price: f32) -> Bid {
        Bid::new(t, self.id, price)
    }

    pub fn approve(&self, candidate: &mut Candidate) -> Result<ApprovalResult, UserError> {
        candidate.approve(&self.id).map_err(|_| UserError::Error)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("")]
    Error,
}
