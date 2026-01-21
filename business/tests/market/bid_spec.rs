use cucumber::{then, when};
use error_stack::Report;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{Money, iso};
use thiserror::Error;
use tracing::instrument;

use crate::market_world::MarketWorld;

#[derive(Debug, Error)]
#[error("error in test")]
struct TestError;

#[when(expr = "{word} sends a bid order not above {float}")]
fn send_bid_order(world: &mut MarketWorld, user: String, price: f32) {
    let user_id = world.buyers.get(&user).unwrap();

    world.market.bid(user_id, price);
}

#[then(regex = r"^the market has (\d) bid orders?$")]
#[instrument(err)]
fn one_bid_order(world: &mut MarketWorld, num_orders: usize) -> Result<(), Report<TestError>> {
    assert_eq!(world.market.number_of_bids(), num_orders);

    Ok(())
}

#[then(expr = "buy price equals {float}")]
fn buy_price_equals(world: &mut MarketWorld, price: f32) {
    let target = Money::from_decimal(Decimal::from_f32(price).unwrap(), iso::EUR);

    assert_eq!(world.market.buy_price().unwrap(), target);
}
