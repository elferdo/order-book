use std::collections::HashMap;

use cucumber::{World, given};
use error_stack::{Report, ResultExt};
use model::{market::Market, user::user::User};
use repositories::Repository;
use sqlx::{PgPool, query};
use thiserror::Error;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(World, Debug, Default)]
pub struct MarketWorld {
    pub market: Market,
    pub sellers: HashMap<String, Uuid>,
    pub buyers: HashMap<String, Uuid>,
    pub pool: Option<PgPool>,
}

#[derive(Error, Debug)]
enum TestError {
    #[error("acquire error")]
    AcquireError,

    #[error("insert user error")]
    InsertUserError,
}

#[given(expr = "a seller named {word}")]
#[instrument(err(Debug))]
async fn add_seller(world: &mut MarketWorld, user: String) -> Result<(), Report<TestError>> {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let id = Uuid::new_v7(timestamp);

    world.sellers.insert(user, id);

    let mut t = world
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .change_context(TestError::AcquireError)?;

    query!("INSERT INTO \"user\" VALUES ($1);", id)
        .execute(&mut *t)
        .await
        .change_context(TestError::InsertUserError)?;

    Ok(())
}

#[given(expr = "a buyer named {word}")]
#[instrument(err(Debug))]
async fn add_buyer(world: &mut MarketWorld, user: String) -> Result<(), Report<TestError>> {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let id = Uuid::new_v7(timestamp);

    world.buyers.insert(user, id);

    let mut t = world
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .change_context(TestError::AcquireError)?;

    query!("INSERT INTO \"user\" VALUES ($1);", id)
        .execute(&mut *t)
        .await
        .change_context(TestError::InsertUserError)?;

    Ok(())
}

#[given("an empty market")]
fn empty_market(_: &mut MarketWorld) {}
