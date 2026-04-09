pub mod candidate;
pub mod candidate_repository;
pub mod candidate_repository_impl;
pub mod deal;
pub mod deal_repository;
pub mod deal_repository_impl;
pub mod market_repository_impl;
pub mod repository;
pub mod repository_error;

use error_stack::{Report, ResultExt};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{
    Money,
    iso::{self, Currency},
};
use sqlx::{PgConnection, PgPool};
use thiserror::Error;
use tracing::instrument;
use uuid::{ContextV7, Timestamp};

use order::{ask::Ask, bid::Bid};

use crate::{
    candidate::Candidate, candidate_repository::CandidateRepository, repository::MarketRepository,
    repository_error::RepositoryError,
};

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
    pub fn new(asks: Vec<Ask>, bids: Vec<Bid>) -> Self {
        Self { asks, bids }
    }

    #[instrument(err(Debug), skip(self))]
    pub fn run(&mut self, timestamp: Timestamp) -> Result<Vec<Candidate>, Report<MarketError>> {
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

    pub fn sell_price<'c>(&self) -> Option<Money<'c, Currency>> {
        /*
        self.asks
            .iter()
            .map(|a| Money::from_decimal(Decimal::from_f32(a.get_price()).unwrap(), iso::EUR))
            .min()
            */
        Some(Money::from_decimal(
            Decimal::from_f32(3.33).unwrap(),
            iso::EUR,
        ))
    }

    pub fn buy_price<'c>(&self) -> Option<Money<'c, Currency>> {
        /*
                self.bids
                    .iter()
                    .map(|a| Money::from_decimal(Decimal::from_f32(a.get_price()).unwrap(), iso::EUR))
                    .max()
        */
        Some(Money::from_decimal(
            Decimal::from_f32(3.33).unwrap(),
            iso::EUR,
        ))
    }

    pub fn number_of_asks(&self) -> usize {
        self.asks.len()
    }

    pub fn number_of_bids(&self) -> usize {
        self.bids.len()
    }
}

#[instrument(err(Debug), skip(conn))]
pub async fn market_step(conn: &mut PgConnection) -> Result<(), Report<MarketError>> {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let asks = conn
        .get_unbound_asks()
        .await
        .change_context(MarketError::Error)?;

    let bids = conn
        .get_unbound_bids()
        .await
        .change_context(MarketError::Error)?;

    let mut market = Market::new(asks, bids);

    let candidates = market.run(timestamp).change_context(MarketError::Error)?;

    conn.persist_candidates(candidates.into_iter())
        .await
        .change_context(MarketError::CandidatePersistanceError)?;

    Ok(())
}
