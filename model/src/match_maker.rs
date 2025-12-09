use tracing::{debug, info, instrument};

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
        let matches: Vec<_> = orders
            .into_iter()
            .map(|a| Match::new(*a.get_id(), *order.get_id()))
            .collect();

        repository.persist_order_matches(matches).await.unwrap();

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
