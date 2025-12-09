use tracing::{debug, info, instrument};

use crate::{
    ask::Ask,
    bid::Bid,
    repository::{AskRepository, BidRepository},
};

#[instrument(skip(ask_repository))]
pub async fn find_matches_for_bid(ask_repository: &mut impl AskRepository, bid: &Bid) {
    if let Ok(_asks) = ask_repository.find_asks_below(bid.get_price()).await {
        info!("processing matching asks for bid");
    } else {
        debug!("no matching asks for bid");
    }
}

#[instrument(skip(bid_repository))]
pub async fn find_matches_for_ask(bid_repository: &mut impl BidRepository, ask: &Ask) {
    if let Ok(_asks) = bid_repository.find_bids_above(ask.get_price()).await {
        info!("processing matching asks for ask");
    } else {
        debug!("no matching asks for ask");
    }
}
