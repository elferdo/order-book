use cucumber::{given, then, when};
use error_stack::{Report, ResultExt};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{Money, iso};
use sqlx::query;
use thiserror::Error;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::market_world::MarketWorld;

#[derive(Debug, Error)]
#[error("error in test")]
struct TestError;

#[given(expr = "a bid order not above {float} by {word}")]
#[instrument(err(Debug))]
async fn send_bid_order(
    world: &mut MarketWorld,
    price: f32,
    user: String,
) -> Result<(), Report<TestError>> {
    let user_id = world.buyers.get(&user).ok_or(TestError)?;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let id = Uuid::new_v7(timestamp);

    let mut t = world
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .change_context(TestError {})?;

    query!("INSERT INTO bid VALUES ($1, $2, $3);", id, user_id, price)
        .execute(&mut *t)
        .await
        .change_context(TestError {})?;

    Ok(())
}

#[then(regex = r"^the market has (\d) bid orders?$")]
#[instrument(err(Debug))]
fn one_bid_order(world: &mut MarketWorld, num_orders: usize) -> Result<(), Report<TestError>> {
    assert_eq!(world.market.number_of_bids(), num_orders);

    Ok(())
}

#[then(expr = "buy price equals {float}")]
fn buy_price_equals(world: &mut MarketWorld, price: f32) {
    let target = Money::from_decimal(Decimal::from_f32(price).unwrap(), iso::EUR);

    assert_eq!(world.market.buy_price().unwrap(), target);
}
