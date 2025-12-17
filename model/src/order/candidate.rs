use uuid::{Timestamp, Uuid};

use crate::order::{ask::Ask, bid::Bid};
use crate::repository_error::RepositoryError;

#[derive(Debug)]
pub struct Candidate {
    id: Uuid,
    ask: Ask,
    bid: Bid,
    approval: Approval,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Approval {
    pub ask: bool,
    pub bid: bool,
}

impl Approval {
    fn both_approved(&self) -> bool {
        self.ask == self.bid
    }
}

pub enum ApprovalResult {
    Partial,
    Complete,
}

impl Candidate {
    pub fn new(t: Timestamp, ask: Ask, bid: Bid) -> Self {
        let id = Uuid::new_v7(t);

        Self {
            id,
            ask,
            bid,
            approval: Approval::default(),
        }
    }

    pub fn with(id: Uuid, ask: Ask, bid: Bid, approval: Approval) -> Self {
        Self {
            id,
            ask,
            bid,
            approval,
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_ask(&self) -> &Ask {
        &self.ask
    }

    pub fn get_bid(&self) -> &Bid {
        &self.bid
    }

    pub fn get_buyer_id(&self) -> &Uuid {
        self.bid.get_user_id()
    }

    pub fn get_seller_id(&self) -> &Uuid {
        &self.ask.get_user_id()
    }

    pub fn get_price(&self) -> f32 {
        (self.ask.get_price() + self.bid.get_price()) / 2.0
    }

    pub fn get_ask_approval(&self) -> bool {
        self.approval.ask
    }

    pub fn get_bid_approval(&self) -> bool {
        self.approval.bid
    }

    pub async fn approve(&mut self, user_id: &Uuid) -> Result<ApprovalResult, RepositoryError> {
        if *user_id == *self.ask.get_user_id() {
            self.approval.ask = true;
        } else if *user_id == *self.bid.get_user_id() {
            self.approval.bid = true;
        } else {
            return Err(RepositoryError::UnexpectedResult);
        }

        if self.approval.both_approved() {
            Ok(ApprovalResult::Complete)
        } else {
            Ok(ApprovalResult::Partial)
        }
    }
}
