use std::collections::HashMap;

use cucumber::{World, given, then, when};
use error_stack::{Report, ResultExt};
use model::order::candidate_repository::CandidateRepository;
use model::user::repository::UserRepository;
use model::{market::Market, user::user::User};
use repositories::Repository;
use sqlx::{PgPool, query};
use thiserror::Error;
use tracing::{debug, info, instrument};
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

    #[error("transaction error")]
    TransactionError,

    #[error("error in test")]
    Error,
}

#[given(expr = "a seller named {word}")]
#[instrument(err(Debug))]
async fn add_seller(world: &mut MarketWorld, user: String) -> Result<(), Report<TestError>> {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let id = Uuid::new_v7(timestamp);

    world.sellers.insert(user.clone(), id);

    let mut t = world
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .change_context(TestError::AcquireError)?;

    debug!("inserting {user}");

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

#[when(expr = "market runs")]
#[instrument(err(Debug))]
async fn run_market(world: &mut MarketWorld) -> Result<(), Report<TestError>> {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut t = world
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .change_context(TestError::TransactionError)?;

    let mut repo = Repository::new(&mut t).await;

    world
        .market
        .run(timestamp, &mut repo)
        .await
        .change_context(TestError::Error)?;

    Ok(())
}

#[then(expr = "{word} {word} has {int} candidates")]
#[instrument(err(Debug))]
async fn user_has_candidates(
    world: &mut MarketWorld,
    user_role: String,
    user_name: String,
    num_candidates: usize,
) -> Result<(), Report<TestError>> {
    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut t = world
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .change_context(TestError::TransactionError)?;

    let mut repo = Repository::new(&mut t).await;

    let user_id = match user_role.as_str() {
        "buyer" => world.buyers.get(&user_name).ok_or(TestError::Error)?,
        "seller" => world.sellers.get(&user_name).ok_or(TestError::Error)?,
        _ => Err(TestError::Error)?,
    };

    let user = repo
        .find_user(user_id)
        .await
        .change_context(TestError::Error)?;

    let candidates = repo
        .find_candidates_by_user(&user)
        .await
        .change_context(TestError::Error)?;

    assert_eq!(candidates.len(), num_candidates);

    Ok(())
}
