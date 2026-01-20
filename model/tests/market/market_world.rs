use std::collections::HashMap;

use cucumber::{World, given};
use model::market::Market;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(World, Debug, Default)]
pub struct MarketWorld {
    pub market: Market,
    pub sellers: HashMap<String, Uuid>,
    pub buyers: HashMap<String, Uuid>,
}

#[given(expr = "a seller named {word}")]
fn add_seller(world: &mut MarketWorld, user: String) {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let id = Uuid::new_v7(timestamp);

    world.sellers.insert(user, id);
}

#[given(expr = "a buyer named {word}")]
fn add_buyer(world: &mut MarketWorld, user: String) {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let id = Uuid::new_v7(timestamp);

    world.buyers.insert(user, id);
}

#[given("an empty market")]
fn empty_market(_: &mut MarketWorld) {}
