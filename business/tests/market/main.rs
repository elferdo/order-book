use cucumber::{
    World as _,
    event::ScenarioFinished,
    gherkin::{Feature, Rule, Scenario},
};
use sqlx::{PgPool, QueryBuilder};
use testcontainers_modules::testcontainers::runners::AsyncRunner;

use crate::market_world::MarketWorld;

mod ask_spec;
mod bid_spec;
mod market_world;
mod match_making;

async fn setup(
    connection_string_base: String,
    pool: PgPool,
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    world: &mut MarketWorld,
) {
    let _ = QueryBuilder::new("CREATE DATABASE TEST;")
        .build()
        .execute(&pool)
        .await;

    let connection_string = format!("{connection_string_base}/test");

    let pool = PgPool::connect(&connection_string).await.unwrap();

    world.pool = Some(pool);
}

async fn teardown(
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    _ev: &ScenarioFinished,
    _world: Option<&mut MarketWorld>,
) {
    let pool = _world.unwrap().pool.as_ref().unwrap();

    let _ = QueryBuilder::new("DROP DATABASE TEST;")
        .build()
        .execute(pool)
        .await;
}

#[tokio::main]
async fn main() {
    let container = testcontainers_modules::postgres::Postgres::default()
        .start()
        .await
        .unwrap();

    let connection_string_base = format!(
        "postgres://postgres:postgres@127.0.0.1:{}",
        container.get_host_port_ipv4(5432).await.unwrap()
    );

    let connection_string = format!("{connection_string_base}/postgres");

    let pool = PgPool::connect(&connection_string).await.unwrap();

    let features = vec![
        "tests/features/ask.feature",
        "tests/features/ask.feature",
        "tests/features/ask.feature",
    ];

    for feature in features {
        let connection_string_base_before = format!(
            "postgres://postgres:postgres@127.0.0.1:{}",
            container.get_host_port_ipv4(5432).await.unwrap()
        );

        let pool = pool.clone();

        MarketWorld::cucumber()
            .before(move |f, r, s, w| {
                Box::pin(setup(
                    connection_string_base_before.clone(),
                    pool.clone(),
                    f,
                    r,
                    s,
                    w,
                ))
            })
            .after(move |f, r, s, ev, w| Box::pin(teardown(f, r, s, ev, w)))
            .run(feature)
            .await;
    }
}
