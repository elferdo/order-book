use std::{cmp::Ordering, collections::BTreeSet};

use tracing::{error, info, instrument};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::{
    lock_mode::LockMode,
    order::{
        order_match::Match,
        repository::{OrderRepository, OrderRepositoryError},
    },
    repository::OrderMatchRepository,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ask {
    id: Uuid,
    seller: Uuid,
    not_below: f32,
}

impl Ask {
    pub fn new(t: Timestamp, user_id: Uuid, price: f32) -> Self {
        let id = Uuid::new_v7(t);

        Self {
            id,
            seller: user_id,
            not_below: price,
        }
    }

    pub fn with(id: Uuid, user_id: Uuid, price: f32) -> Self {
        Self {
            id,
            seller: user_id,
            not_below: price,
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_user_id(&self) -> Uuid {
        self.seller
    }

    pub fn get_price(&self) -> f32 {
        self.not_below
    }

    #[instrument(skip(repository))]
    pub async fn generate_matches<R>(&self, repository: &mut R) -> Result<(), OrderRepositoryError>
    where
        R: OrderRepository + OrderMatchRepository,
    {
        let mut matching_orders: BTreeSet<_> = repository
            .find_bids_above(LockMode::KeyShare, self.get_price())
            .await?
            .into_iter()
            .collect();

        if matching_orders.is_empty() {
            return Ok(());
        };

        let first = matching_orders.pop_first().expect("this should never fail");

        let context = ContextV7::new();
        let t = Timestamp::now(context);

        let order_match = Match::new(t, *self, first);

        if let Err(e) = repository.persist_order_matches([order_match]).await {
            match e {
                crate::repository::OrderMatchRepositoryError::DatabaseError => {
                    error!("{e}");
                }
                crate::repository::OrderMatchRepositoryError::UserError => todo!(),
            }
        };

        info!("processing matching orders for ask");

        Ok(())
    }
}

impl PartialOrd for Ask {
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

impl Eq for Ask {}

impl Ord for Ask {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(c) = self.partial_cmp(other) {
            c
        } else {
            // If we can't establish a priority, let's just give both orders equal priority
            Ordering::Equal
        }
    }
}
