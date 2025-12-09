use tracing::{debug, info, instrument};

use crate::{
    ask::Ask,
    bid::Bid,
    lock_mode::LockMode,
    order_match::Match,
    repository::{AskRepository, BidRepository, OrderMatchRepository},
};

#[instrument(skip(repository))]
pub async fn find_matches_for_bid<R>(repository: &mut R, bid: &Bid)
where
    R: AskRepository + OrderMatchRepository,
{
    if let Ok(_asks) = repository
        .find_asks_below(LockMode::KeyShare, bid.get_price())
        .await
    {
        let matches: Vec<_> = _asks
            .into_iter()
            .map(|a| Match::new(*a.get_id(), *bid.get_id()))
            .collect();

        repository.persist_order_matches(matches).await.unwrap();

        info!("processing matching asks for bid");
    } else {
        debug!("no matching asks for bid");
    }
}

#[instrument(skip(bid_repository))]
pub async fn find_matches_for_ask(bid_repository: &mut impl BidRepository, ask: &Ask) {
    if let Ok(_asks) = bid_repository
        .find_bids_above(LockMode::KeyShare, ask.get_price())
        .await
    {
        info!("processing matching asks for ask");
    } else {
        debug!("no matching asks for ask");
    }
}
