use crate::{ask::Ask, bid::Bid};

pub trait AskRepository {
    fn find_asks_below(price: f32) -> Vec<Ask>;
}

pub async fn find_matches_for_bid<A: AskRepository>(bid: &Bid) {
    let asks = A::find_asks_below(bid.get_price());

    todo!()
}
