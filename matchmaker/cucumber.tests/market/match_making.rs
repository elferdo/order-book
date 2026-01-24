use cucumber::{World, given, then};
use error_stack::Report;
use model::market::Market;
use thiserror::Error;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::market_world::MarketWorld;

#[derive(Debug, Error)]
#[error("error in test")]
struct TestError;

#[then(expr = "buyer {word} has {int} candidate matching seller {word}'s ask")]
fn buyer_has_candidate_matching(
    world: &mut MarketWorld,
    user: String,
    number: u8,
) -> Result<(), Report<TestError>> {
    let buyer_id = world.buyers.get(&user).unwrap();

    let market = &world.market;

    Ok(())
}

#[then(expr = "seller {word} has {int} candidate matching buyer {word}'s bid")]
fn g(world: &mut MarketWorld, user: String, number: u8) {}
