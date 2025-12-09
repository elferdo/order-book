use thiserror::Error;

use crate::{ask::Ask, bid::Bid};

pub trait AskRepository {
    fn find_asks_below(
        &mut self,
        price: f32,
    ) -> impl Future<Output = Result<Vec<Ask>, AskRepositoryError>>;
}

pub async fn find_matches_for_bid(ask_repository: &mut impl AskRepository, bid: &Bid) {
    let _asks = ask_repository.find_asks_below(bid.get_price()).await;

    todo!()
}

#[derive(Debug, Error)]
pub enum AskRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}
