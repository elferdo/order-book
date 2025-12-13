use std::collections::BTreeSet;
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
pub async fn find_matches_for_ask<R>(
    repository: &mut R,
    ask: Ask,
) -> Result<(), OrderRepositoryError>
where
    R: OrderRepository + OrderMatchRepository,
{
    let mut matching_orders: BTreeSet<_> = repository
        .find_bids_above(LockMode::KeyShare, ask.get_price())
        .await?
        .into_iter()
        .collect();

    if matching_orders.is_empty() {
        return Ok(());
    };

    let first = matching_orders.pop_first().expect("this should never fail");

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

    Ok(())
}

#[instrument(skip(repository))]
pub async fn find_matches_for_bid<R>(
    repository: &mut R,
    bid: Bid,
) -> Result<(), OrderRepositoryError>
where
    R: OrderRepository + OrderMatchRepository,
{
    let mut matching_orders: BTreeSet<_> = repository
        .find_asks_below(LockMode::KeyShare, bid.get_price())
        .await?
        .into_iter()
        .collect();

    if matching_orders.is_empty() {
        return Ok(());
    }

    let first = matching_orders.pop_first().expect("this should never fail");

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

    Ok(())
}
