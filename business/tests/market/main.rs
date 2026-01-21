use cucumber::{
    World as _,
    event::ScenarioFinished,
    gherkin::{Feature, Rule, Scenario},
};
use error_stack::{IntoReport, Report, ResultExt};
use sqlx::{PgPool, QueryBuilder, query};
use testcontainers_modules::testcontainers::{ImageExt, runners::AsyncRunner};
use thiserror::Error;
use tracing::{error, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::market_world::MarketWorld;

mod ask_spec;
mod bid_spec;
mod market_world;
mod match_making;

#[derive(Error, Debug)]
#[error("error running test")]
struct TestError;

#[instrument(err(Debug))]
async fn setup(
    connection_string_base: String,
    pool: PgPool,
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    world: &mut MarketWorld,
) -> Result<(), Report<TestError>> {
    let _ = QueryBuilder::new("CREATE DATABASE TEST;")
        .build()
        .execute(&pool)
        .await;

    let connection_string = format!("{connection_string_base}/test");

    let pool = PgPool::connect(&connection_string).await.unwrap();

    let mut t = pool.acquire().await.change_context(TestError)?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .change_context(TestError {})?;

    world.pool = Some(pool);

    Ok(())
}

async fn setup_callback(
    connection_string_base: String,
    pool: PgPool,
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    world: &mut MarketWorld,
) {
    if let Err(e) = setup(
        connection_string_base,
        pool,
        _feature,
        _rule,
        _scenario,
        world,
    )
    .await
    {
        // error!("{e:#?}")
        error!("{e}")
    }
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
async fn main() -> Result<(), Report<TestError>> {
    Registry::default()
        .with(ErrorLayer::default())
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().pretty())
        .init();

    let container = testcontainers_modules::postgres::Postgres::default()
        .with_tag("17-alpine")
        .start()
        .await
        .unwrap();

    let connection_string_base = format!(
        "postgres://postgres:postgres@127.0.0.1:{}",
        container.get_host_port_ipv4(5432).await.unwrap()
    );

    let connection_string = format!("{connection_string_base}/postgres");

    let pool = PgPool::connect(&connection_string).await.unwrap();

    let mut t = pool.acquire().await.change_context(TestError)?;

    query!("CREATE USER fernando SUPERUSER;")
        .execute(&mut *t)
        .await
        .change_context(TestError)?;

    let features = vec!["tests/features/match_making.feature"];

    for feature in features {
        let connection_string_base_before = format!(
            "postgres://postgres:postgres@127.0.0.1:{}",
            container.get_host_port_ipv4(5432).await.unwrap()
        );

        let pool = pool.clone();

        MarketWorld::cucumber()
            .before(move |f, r, s, w| {
                Box::pin(setup_callback(
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

    Ok(())
}
