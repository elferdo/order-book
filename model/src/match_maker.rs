use tracing::{debug, error, info, instrument};
use uuid::{ContextV7, Timestamp};

use crate::{
    lock_mode::LockMode,
    order::Order,
    order_match::Match,
    repository::{OrderMatchRepository, OrderRepository, OrderRepositoryError},
};

#[instrument(skip(repository))]
pub async fn find_matches_for_order<R>(repository: &mut R, order: &Order)
where
    R: OrderRepository + OrderMatchRepository,
{
    if let Ok(orders) = find_matching_orders(repository, order).await {
        let context = ContextV7::new();
        let t = Timestamp::now(context);

        let matches: Vec<_> = orders
            .into_iter()
            .map(|o| match order {
                Order::Bid { id, .. } => Match::new(t, *o.get_id(), *id),
                Order::Ask { id, .. } => Match::new(t, *id, *o.get_id()),
            })
            .collect();

        if let Err(e) = repository.persist_order_matches(matches).await {
            match e {
                crate::repository::OrderMatchRepositoryError::DatabaseError => {
                    error!("{e}");
                }
                crate::repository::OrderMatchRepositoryError::UserError => todo!(),
            }
        };

        info!("processing matching orders for {order:?}");
    } else {
        debug!("no matching orders for {order:?}");
    }
}

async fn find_matching_orders<R>(
    repository: &mut R,
    order: &Order,
) -> Result<Vec<Order>, OrderRepositoryError>
where
    R: OrderRepository + OrderMatchRepository,
{
    match order {
        Order::Ask { .. } => {
            repository
                .find_bids_above(LockMode::KeyShare, order.get_price())
                .await
        }
        Order::Bid { .. } => {
            repository
                .find_asks_below(LockMode::KeyShare, order.get_price())
                .await
        }
    }
}
