use cucumber::{given, then, when};
use error_stack::{Report, ResultExt};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{Money, iso};
use sqlx::query;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::market::{cucumber_error::CucumberError, market_world::MarketWorld};

#[given(expr = "a bid order not above {float} by {word}")]
#[instrument(err(Debug))]
pub async fn send_bid_order(
    world: &mut MarketWorld,
    price: f32,
    user: String,
) -> Result<(), Report<CucumberError>> {
    let buyer_id = world.buyers[&user];

    user::new_bid(world.pool.as_ref().unwrap().clone(), buyer_id, price)
        .await
        .change_context(CucumberError::Error)?;

    /*
    let user_id = world
        .buyers
        .get(&user)
        .ok_or(CucumberError::BuyerNotFound)?;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let id = Uuid::new_v7(timestamp);

    let mut t = world
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .change_context(CucumberError::Error)?;

    query!("INSERT INTO bid VALUES ($1, $2, $3);", id, user_id, price)
        .execute(&mut *t)
        .await
        .change_context(CucumberError::Error)?;
    */

    Ok(())
}

#[then(regex = r"^the market has (\d) bid orders?$")]
#[instrument(err(Debug))]
fn one_bid_order(world: &mut MarketWorld, num_orders: usize) -> Result<(), Report<CucumberError>> {
    assert_eq!(world.market.number_of_bids(), num_orders);

    Ok(())
}

#[then(expr = "buy price equals {float}")]
fn buy_price_equals(world: &mut MarketWorld, price: f32) {
    let target = Money::from_decimal(Decimal::from_f32(price).unwrap(), iso::EUR);

    assert_eq!(world.market.buy_price().unwrap(), target);
}
