use std::collections::HashMap;

use cucumber::{World, given, then, when};
use error_stack::Report;
use model::market::Market;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{Money, iso};
use thiserror::Error;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(World, Debug, Default)]
pub struct MarketWorld {
    market: Market,
    sellers: HashMap<String, Uuid>,
}

#[derive(Debug, Error)]
#[error("error in test")]
struct TestError;

#[given(expr = "a seller named {word}")]
fn add_seller(world: &mut MarketWorld, user: String) {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let id = Uuid::new_v7(timestamp);

    world.sellers.insert(user, id);
}

#[given("an empty market")]
fn empty_market(_: &mut MarketWorld) {}

#[when(expr = "{word} sends an ask order not below {float}")]
fn send_ask_order(world: &mut MarketWorld, user: String, price: f32) {
    let user_id = world.sellers.get(&user).unwrap();

    world.market.ask(user_id, price);
}

#[then(regex = r"^the market has (\d) ask orders?$")]
#[instrument(err)]
fn one_ask_order(world: &mut MarketWorld, num_orders: usize) -> Result<(), Report<TestError>> {
    assert_eq!(world.market.number_of_asks(), num_orders);

    Ok(())
}

#[then(expr = "sell price equals {float}")]
fn sell_price_equals(world: &mut MarketWorld, price: f32) {
    let target = Money::from_decimal(Decimal::from_f32(price).unwrap(), iso::EUR);

    assert_eq!(world.market.sell_price().unwrap(), target);
}
