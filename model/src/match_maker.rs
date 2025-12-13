use std::{collections::BTreeSet, sync::Arc};
use tracing::{debug, error, info, instrument};
use uuid::{ContextV7, Timestamp};

use crate::{
    ask::Ask,
    bid::Bid,
    lock_mode::LockMode,
    order_match::Match,
    repository::{OrderMatchRepository, OrderRepository, OrderRepositoryError},
};

#[instrument(skip(repository))]
pub async fn find_matches_for_ask<R>(repository: &mut R, ask: Arc<Ask>)
where
    R: OrderRepository + OrderMatchRepository,
{
    if let Ok(mut matching_orders) = find_bids_for_ask(repository, &ask).await {
        let first = match matching_orders.pop_first() {
            Some(m) => Arc::new(m),
            None => return,
        };

        let context = ContextV7::new();
        let t = Timestamp::now(context);

        let order_match = Match::new(t, ask, first);

        if let Err(e) = repository.persist_order_matches([order_match]).await {
            match e {
                crate::repository::OrderMatchRepositoryError::DatabaseError => {
                    error!("{e}");
                }
                crate::repository::OrderMatchRepositoryError::UserError => todo!(),
            }
        };

        info!("processing matching orders for ask");
    } else {
        debug!("no matching orders for ask");
    }
}

#[instrument(skip(repository))]
pub async fn find_matches_for_bid<R>(repository: &mut R, bid: Arc<Bid>)
where
    R: OrderRepository + OrderMatchRepository,
{
    if let Ok(mut matching_orders) = find_asks_for_bid(repository, &bid).await {
        let first = match matching_orders.pop_first() {
            Some(m) => Arc::new(m),
            None => return,
        };

        let context = ContextV7::new();
        let t = Timestamp::now(context);

        let order_match = Match::new(t, first, bid);

        if let Err(e) = repository.persist_order_matches([order_match]).await {
            match e {
                crate::repository::OrderMatchRepositoryError::DatabaseError => {
                    error!("{e}");
                }
                crate::repository::OrderMatchRepositoryError::UserError => todo!(),
            }
        };

        info!("processing matching orders for bid");
    } else {
        debug!("no matching orders for bid");
    }
}

#[instrument(skip(repository))]
pub async fn find_asks_for_bid<R>(
    repository: &mut R,
    order: &Bid,
) -> Result<BTreeSet<Ask>, OrderRepositoryError>
where
    R: OrderRepository + OrderMatchRepository,
{
    debug!("entering find_asks_for_bid");

    let result = repository
        .find_asks_below(LockMode::KeyShare, order.get_price())
        .await?
        .into_iter();

    Ok(BTreeSet::from_iter(result))
}

pub async fn find_bids_for_ask<R>(
    repository: &mut R,
    order: &Ask,
) -> Result<BTreeSet<Bid>, OrderRepositoryError>
where
    R: OrderRepository + OrderMatchRepository,
{
    let result = repository
        .find_bids_above(LockMode::KeyShare, order.get_price())
        .await?
        .into_iter();

    Ok(BTreeSet::from_iter(result))
}
