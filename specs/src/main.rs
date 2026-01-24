mod market;

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

use crate::market::market_world::MarketWorld;

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
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    world: &mut MarketWorld,
) -> Result<(), Report<TestError>> {
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

    query!("CREATE USER fernando SUPERUSER;")
        .execute(&pool)
        .await
        .change_context(TestError::DatabaseUserCreation)?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .change_context(TestError::DatabaseMigration)?;

    world.pool = Some(pool);
    world.container = Some(container);
    world.connection_string_base = connection_string_base;

    Ok(())
}

async fn setup_callback(
    _feature: &Feature,
    _rule: Option<&Rule>,
    _scenario: &Scenario,
    world: &mut MarketWorld,
) {
    if let Err(e) = setup(_feature, _rule, _scenario, world).await {
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

    let mut qb = QueryBuilder::new("DROP DATABASE ");
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

    let features = vec!["tests/features/match_making.feature"];

    for feature in features {
        MarketWorld::cucumber()
            .before(move |f, r, s, w| Box::pin(setup_callback(f, r, s, w)))
            //            .after(move |f, r, s, ev, w| Box::pin(teardown_callback(f, r, s, ev, w)))
            .run(feature)
            .await;
    }

    Ok(())
}
