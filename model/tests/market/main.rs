use cucumber::World as _;

use crate::market_world::MarketWorld;

mod ask_spec;
mod bid_spec;
mod market_world;

#[tokio::main]
async fn main() {
    MarketWorld::cucumber()
        .init_tracing()
        .run("tests/features/ask.feature")
        .await;

    MarketWorld::cucumber()
        .run("tests/features/bid.feature")
        .await;
}
