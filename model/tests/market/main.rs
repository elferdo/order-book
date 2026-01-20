use cucumber::{
    World as _,
    gherkin::{Feature, Rule, Scenario},
};
use sqlx::PgPool;
use testcontainers_modules::testcontainers::runners::AsyncRunner;

use crate::market_world::MarketWorld;

mod ask_spec;
mod bid_spec;
mod market_world;
mod match_making;

async fn init(
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    world: &mut MarketWorld,
) {
    let container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    );

    let pool = PgPool::connect(&connection_string).await.unwrap();

    world.pool = Some(pool);
}

#[tokio::main]
async fn main() {
    MarketWorld::cucumber()
        .before(|f, r, s, w| Box::pin(init(f, r, s, w)))
        .init_tracing()
        .run("tests/features/ask.feature")
        .await;

    MarketWorld::cucumber()
        .run("tests/features/bid.feature")
        .await;

    MarketWorld::cucumber()
        .run("tests/features/match_making.feature")
        .await;
}
