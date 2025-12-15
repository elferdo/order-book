use std::{cmp::Ordering, collections::BTreeSet};

use tracing::{error, info, instrument};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::order::order_match_repository::OrderMatchRepository;
use crate::order::order_match_repository::OrderMatchRepositoryError;
use crate::order::repository::{OrderRepository, OrderRepositoryError};
use crate::{lock_mode::LockMode, order::order_match::Match};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bid {
    id: Uuid,
    buyer: Uuid,
    not_above: f32,
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

    pub fn get_user_id(&self) -> Uuid {
        self.buyer
    }

    pub fn get_price(&self) -> f32 {
        self.not_above
    }

    #[instrument(skip(repository))]
    pub async fn generate_matches<R>(&self, repository: &mut R) -> Result<(), OrderRepositoryError>
    where
        R: OrderRepository + OrderMatchRepository,
    {
        let mut matching_orders: BTreeSet<_> = repository
            .find_asks_below(LockMode::KeyShare, self.get_price())
            .await?
            .into_iter()
            .collect();

        if matching_orders.is_empty() {
            return Ok(());
        }

        let first = matching_orders.pop_first().expect("this should never fail");

        let context = ContextV7::new();
        let t = Timestamp::now(context);

        let order_match = Match::new(t, first, *self);

        if let Err(e) = repository.persist_order_matches([order_match]).await {
            match e {
                OrderMatchRepositoryError::DatabaseError => {
                    error!("{e}");
                }
                OrderMatchRepositoryError::UserError => todo!(),
            }
        };

        info!("processing matching orders for bid");

        Ok(())
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.get_price() < other.get_price() {
            Some(Ordering::Less)
        } else if self.get_price() == other.get_price() {
            Some(Ordering::Equal)
        } else if self.get_price() > other.get_price() {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl Eq for Bid {}

impl Ord for Bid {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(c) = self.partial_cmp(other) {
            c
        } else {
            // If we can't establish a priority, let's just give both orders equal priority
            Ordering::Equal
        }
    }
}
