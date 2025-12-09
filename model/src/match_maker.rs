use tracing::{debug, info, instrument};

use crate::{bid::Bid, repository::AskRepository};

#[instrument(skip(ask_repository))]
pub async fn find_matches_for_bid(ask_repository: &mut impl AskRepository, bid: &Bid) {
    if let Ok(_asks) = ask_repository.find_asks_below(bid.get_price()).await {
        info!("processing matching asks for bid");
    } else {
        debug!("no matching asks for bid");
    }
}
