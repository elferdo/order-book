use thiserror::Error;
use tracing::{debug, instrument};

use crate::{ask::Ask, bid::Bid};

pub trait AskRepository {
    fn find_asks_below(
        &mut self,
        price: f32,
    ) -> impl Future<Output = Result<Vec<Ask>, AskRepositoryError>>;
}

#[instrument(skip(ask_repository))]
pub async fn find_matches_for_bid(ask_repository: &mut impl AskRepository, bid: &Bid) {
    if let Ok(_asks) = ask_repository.find_asks_below(bid.get_price()).await {
        todo!();
    } else {
        debug!("no matching asks for bid");
    }
}

#[derive(Debug, Error)]
pub enum AskRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}
