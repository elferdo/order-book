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

#[derive(Debug, Default)]
pub struct Market {
    asks: Vec<Ask>,
}

impl Market {
    pub fn new() -> Self {
        let asks = Vec::new();

        Self { asks }
    }

    pub fn sell_price(&self) -> Option<Money<Currency>> {
        self.asks
            .iter()
            .map(|a| Money::from_decimal(Decimal::from_f32(a.not_below).unwrap(), iso::EUR))
            .min()
    }

    pub fn number_of_asks(&self) -> usize {
        self.asks.len()
    }

    pub fn ask(&mut self, user: &Uuid, price: f32) {
        let ask = Ask {
            user: *user,
            not_below: price,
        };

        self.asks.push(ask);
    }
}
