use std::collections::BTreeSet;
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
    if let Ok(matching_orders) = find_matching_orders(repository, order).await {
        let first = match matching_orders.first() {
            Some(m) => m,
            None => return,
        };

        let context = ContextV7::new();
        let t = Timestamp::now(context);

        /*
                let matches: Vec<_> = matching_orders
                    .into_iter()
                    .map(|o| match order {
                        Order::Bid { id, .. } => Match::new(t, *o.get_id(), *id),
                        Order::Ask { id, .. } => Match::new(t, *id, *o.get_id()),
                    })
                    .collect();
        */
        let order_match = match order {
            Order::Bid { id, .. } => Match::new(t, *first.get_id(), *id),
            Order::Ask { id, .. } => Match::new(t, *id, *first.get_id()),
        };

        if let Err(e) = repository.persist_order_matches([order_match]).await {
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
) -> Result<BTreeSet<Order>, OrderRepositoryError>
where
    R: OrderRepository + OrderMatchRepository,
{
    let result = match order {
        Order::Ask { price, .. } => repository
            .find_bids_above(LockMode::KeyShare, *price)
            .await?
            .into_iter(),
        Order::Bid { price, .. } => repository
            .find_asks_below(LockMode::KeyShare, *price)
            .await?
            .into_iter(),
    };

    Ok(BTreeSet::from_iter(result))
}
