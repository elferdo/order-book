use cucumber::{
    World as _,
    event::ScenarioFinished,
    gherkin::{Feature, Rule, Scenario},
};
use error_stack::{IntoReport, Report, ResultExt};
use sqlx::{PgPool, QueryBuilder, query};
use testcontainers_modules::testcontainers::{ImageExt, runners::AsyncRunner};
use thiserror::Error;
use tracing::{error, info, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::market_world::MarketWorld;

mod ask_spec;
mod bid_spec;
mod market_world;
mod match_making;

#[derive(Error, Debug)]
#[error("error running test")]
enum TestError {
    #[error("test database setup")]
    DatabaseSetup,

    #[error("test database teardown")]
    DatabaseTeardown,

    #[error("test database connection")]
    DatabaseConnection,

    #[error("test database migration")]
    DatabaseMigration,

    #[error("test database creating new user")]
    DatabaseUserCreation,

    #[error("building query")]
    BuildingQuery,
}

#[instrument(err(Debug))]
async fn setup(
    connection_string_base: String,
    pool: PgPool,
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    world: &mut MarketWorld,
) -> Result<(), Report<TestError>> {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let db_id = Uuid::new_v7(timestamp);
    let b = &mut Uuid::encode_buffer();
    let s = db_id.simple().encode_lower(b);

    let mut qb = QueryBuilder::new("CREATE DATABASE ");

    qb.push(format!("\"{s}\""))
        .build()
        .execute(&pool)
        .await
        .change_context(TestError::DatabaseSetup)?;

    info!("setting up database");

    let connection_string = format!("{connection_string_base}/{s}");

    let pool = PgPool::connect(&connection_string)
        .await
        .change_context(TestError::DatabaseConnection)?;

    let mut t = pool
        .acquire()
        .await
        .change_context(TestError::DatabaseConnection)?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .change_context(TestError::DatabaseMigration)?;

    world.pool = Some(pool);
    world.db_id = db_id;
    world.connection_string_base = connection_string_base;

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

#[instrument(err(Debug))]
async fn teardown(
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    _ev: &ScenarioFinished,
    world: Option<&mut MarketWorld>,
) -> Result<(), Report<TestError>> {
    let pool = world.as_ref().unwrap().pool.as_ref().unwrap().clone();
    pool.close().await;

    let connection_string_base = &world.as_ref().unwrap().connection_string_base;

    let mut qb = QueryBuilder::new("DROP DATABASE ");

    let connection_string = format!("{connection_string_base}/postgres");

    let pool = PgPool::connect(&connection_string)
        .await
        .change_context(TestError::DatabaseConnection)?;

    let mut t = pool
        .acquire()
        .await
        .change_context(TestError::DatabaseConnection)?;

    let b = &mut Uuid::encode_buffer();
    let s = world.as_ref().unwrap().db_id.simple().encode_lower(b);

    qb.push(format!("\"{}\"", s))
        .build()
        .execute(&mut *t)
        .await
        .change_context(TestError::DatabaseTeardown)?;

    Ok(())
}

async fn teardown_callback(
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    _ev: &ScenarioFinished,
    _world: Option<&mut MarketWorld>,
) {
    if let Err(e) = teardown(_feature, _rule, _scenario, _ev, _world).await {
        error!("{e}")
    }
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

    let pool = PgPool::connect(&connection_string)
        .await
        .change_context(TestError::DatabaseConnection)?;

    let mut t = pool
        .acquire()
        .await
        .change_context(TestError::DatabaseConnection)?;

    query!("CREATE USER fernando SUPERUSER;")
        .execute(&mut *t)
        .await
        .change_context(TestError::DatabaseUserCreation)?;

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
            .after(move |f, r, s, ev, w| Box::pin(teardown_callback(f, r, s, ev, w)))
            .run(feature)
            .await;
    }

    Ok(())
}
