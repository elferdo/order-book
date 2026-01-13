use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{
    Money,
    iso::{self, Currency},
};
use uuid::Uuid;

#[cfg(test)]
mod tests;

#[derive(Debug)]
struct Ask {
    user: Uuid,
    not_below: f32,
}

#[derive(Debug)]
struct Bid {
    user: Uuid,
    not_above: f32,
}

#[derive(Debug, Default)]
pub struct Market {
    asks: Vec<Ask>,
    bids: Vec<Bid>,
}

impl Market {
    pub fn new() -> Self {
        let asks = Vec::new();
        let bids = Vec::new();

        Self { asks, bids }
    }

    pub fn sell_price(&self) -> Option<Money<Currency>> {
        self.asks
            .iter()
            .map(|a| Money::from_decimal(Decimal::from_f32(a.not_below).unwrap(), iso::EUR))
            .min()
    }

    pub fn buy_price(&self) -> Option<Money<Currency>> {
        self.bids
            .iter()
            .map(|a| Money::from_decimal(Decimal::from_f32(a.not_above).unwrap(), iso::EUR))
            .max()
    }

    pub fn number_of_asks(&self) -> usize {
        self.asks.len()
    }

    pub fn number_of_bids(&self) -> usize {
        self.bids.len()
    }

    pub fn ask(&mut self, user: &Uuid, price: f32) {
        let ask = Ask {
            user: *user,
            not_below: price,
        };

        self.asks.push(ask);
    }

    pub fn bid(&mut self, user: &Uuid, price: f32) {
        let bid = Bid {
            user: *user,
            not_above: price,
        };

        self.bids.push(bid);
    }
}
