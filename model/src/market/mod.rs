pub mod repository;

use error_stack::{Report, ResultExt};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{
    Money,
    iso::{self, Currency},
};
use thiserror::Error;
use tracing::instrument;
use uuid::{Timestamp, Uuid};

use crate::{
    market::repository::MarketRepository,
    order::{ask::Ask, bid::Bid, candidate::Candidate},
    repository_error::RepositoryError,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Default)]
pub struct Market {
    asks: Vec<Ask>,
    bids: Vec<Bid>,
}

#[derive(Debug, Error)]
pub enum MarketError {
    #[error("market error")]
    Error,

    #[error("error persisting candidates")]
    CandidatePersistanceError,
}

impl Market {
    // Let's leave this method non-async and fill the structures in run()
    pub fn new() -> Self {
        let asks = Vec::new();
        let bids = Vec::new();

        Self { asks, bids }
    }

    #[instrument(err(Debug), skip(self, repo))]
    pub async fn run(
        &mut self,
        timestamp: Timestamp,
        repo: &mut impl MarketRepository,
    ) -> Result<(), Report<MarketError>> {
        self.asks = repo
            .get_unbound_asks()
            .await
            .change_context(MarketError::Error)?;

        self.bids = repo
            .get_unbound_bids()
            .await
            .change_context(MarketError::Error)?;

        let candidates = self.do_matching(timestamp)?;

        repo.persist_candidates(candidates)
            .await
            .change_context(MarketError::CandidatePersistanceError)?;

        Ok(())
    }

    fn do_matching(&mut self, timestamp: Timestamp) -> Result<Vec<Candidate>, Report<MarketError>> {
        self.asks.sort_by(Ask::sort_fn);
        self.bids.sort_by(Bid::sort_fn);

        let mut candidates = Vec::new();

        let ask_iter = self.asks.iter();
        let bid_iter = self.bids.iter();

        for (ask, bid) in ask_iter.zip(bid_iter) {
            if bid.get_price() >= ask.get_price() {
                let candidate = Candidate::new(timestamp, *ask, *bid);

                candidates.push(candidate);
            }
        }

        Ok(candidates)
    }

    pub fn sell_price(&self) -> Option<Money<Currency>> {
        self.asks
            .iter()
            .map(|a| Money::from_decimal(Decimal::from_f32(a.get_price()).unwrap(), iso::EUR))
            .min()
    }

    pub fn buy_price(&self) -> Option<Money<Currency>> {
        self.bids
            .iter()
            .map(|a| Money::from_decimal(Decimal::from_f32(a.get_price()).unwrap(), iso::EUR))
            .max()
    }

    pub fn number_of_asks(&self) -> usize {
        self.asks.len()
    }

    pub fn number_of_bids(&self) -> usize {
        self.bids.len()
    }
}
