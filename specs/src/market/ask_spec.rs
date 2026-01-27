use cucumber::{given, then, when};
use error_stack::{Report, ResultExt};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rusty_money::{Money, iso};
use sqlx::query;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::market::{cucumber_error::CucumberError, market_world::MarketWorld};

#[given(expr = "an ask order not below {float} by {word}")]
#[instrument(err(Debug))]
pub async fn send_ask_order(
    world: &mut MarketWorld,
    price: f32,
    user: String,
) -> Result<(), Report<CucumberError>> {
    let seller_id = world.sellers[&user];

    user::new_ask(world.pool.as_ref().unwrap().clone(), seller_id, price)
        .await
        .change_context(CucumberError::Error)?;
    /*
    let user_id = world
        .sellers
        .get(&user)
        .ok_or(CucumberError::SellerNotFound)?;

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

    query!("INSERT INTO ask VALUES ($1, $2, $3);", id, user_id, price)
        .execute(&mut *t)
        .await
        .change_context(CucumberError::Error)?;
    */

    Ok(())
}

#[then(regex = r"^the market has (\d) ask orders?$")]
#[instrument(err(Debug))]
fn one_ask_order(world: &mut MarketWorld, num_orders: usize) -> Result<(), Report<CucumberError>> {
    assert_eq!(world.market.number_of_asks(), num_orders);

    Ok(())
}

#[then(expr = "sell price equals {float}")]
fn sell_price_equals(world: &mut MarketWorld, price: f32) {
    let target = Money::from_decimal(Decimal::from_f32(price).unwrap(), iso::EUR);

    assert_eq!(world.market.sell_price().unwrap(), target);
}
