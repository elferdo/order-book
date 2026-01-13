use cucumber::World as _;

use crate::spec::MarketWorld;

mod spec;

#[tokio::main]
async fn main() {
    MarketWorld::cucumber()
        .init_tracing()
        .run("tests/features/ask.feature")
        .await;
}
