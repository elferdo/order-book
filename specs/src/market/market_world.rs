use std::collections::HashMap;

use cucumber::{World, gherkin::Step, given, then, when};
use error_stack::{IntoReport, Report, ResultExt};
use matchmaker::Market;
use sqlx::{PgPool, query};
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::testcontainers::ContainerAsync;
use tracing::{debug, info, instrument};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::market::ask_spec::send_ask_order;
use crate::market::bid_spec::send_bid_order;
use crate::market::cucumber_error::CucumberError;

#[derive(World, Debug, Default)]
pub struct MarketWorld {
    pub market: Market,
    pub sellers: HashMap<String, Uuid>,
    pub buyers: HashMap<String, Uuid>,
    pub pool: Option<PgPool>,
    pub db_id: Uuid,
    pub connection_string_base: String,
    pub container: Option<ContainerAsync<Postgres>>,
}

#[given(expr = "users")]
#[instrument(err(Debug))]
async fn add_users(world: &mut MarketWorld, step: &Step) -> Result<(), Report<CucumberError>> {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            // NOTE: skip header
            let role = row[0].as_str();
            let name = row[1].clone();

            match role {
                "buyer" => add_buyer(world, name).await?,
                "seller" => add_seller(world, name).await?,
                _ => Err(CucumberError::Error.into_report())?,
            };
        }
    }

    Ok(())
}

#[given(expr = "ask orders")]
#[instrument(err(Debug))]
async fn add_ask_orders(world: &mut MarketWorld, step: &Step) -> Result<(), Report<CucumberError>> {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            // NOTE: skip header
            let not_below = row[0]
                .parse::<f32>()
                .change_context(CucumberError::ParameterParseError)?;
            let name = row[1].clone();

            send_ask_order(world, not_below, name).await?;
        }
    }

    Ok(())
}

#[given(expr = "bid orders")]
#[instrument(err(Debug))]
async fn add_bid_orders(world: &mut MarketWorld, step: &Step) -> Result<(), Report<CucumberError>> {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            // NOTE: skip header
            let not_above = row[0]
                .parse::<f32>()
                .change_context(CucumberError::ParameterParseError)?;
            let name = row[1].clone();

            send_bid_order(world, not_above, name).await?;
        }
    }

    Ok(())
}

#[given(expr = "a seller named {word}")]
#[instrument(err(Debug))]
async fn add_seller(world: &mut MarketWorld, user: String) -> Result<(), Report<CucumberError>> {
    let response = user::new_user(world.pool.as_ref().unwrap().clone())
        .await
        .change_context(CucumberError::Error)?;

    world.sellers.insert(user, response.id);

    Ok(())
}

#[given(expr = "a buyer named {word}")]
#[instrument(err(Debug))]
async fn add_buyer(world: &mut MarketWorld, user: String) -> Result<(), Report<CucumberError>> {
    let response = user::new_user(world.pool.as_ref().unwrap().clone())
        .await
        .change_context(CucumberError::Error)?;

    world.buyers.insert(user, response.id);

    Ok(())
}

#[when(expr = "market runs")]
#[instrument(err(Debug), skip(world))]
async fn run_market(world: &mut MarketWorld) -> Result<(), Report<CucumberError>> {
    let mut t = world
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .change_context(CucumberError::Error)?;

    matchmaker::market_step(&mut t)
        .await
        .change_context(CucumberError::Error)?;

    Ok(())
}

#[then(expr = "users match")]
#[instrument(err(Debug), skip(world))]
async fn users_match(world: &mut MarketWorld, step: &Step) -> Result<(), Report<CucumberError>> {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            // NOTE: skip header
            let role_a = row[0].clone();
            let name_a = row[1].clone();
            let role_b = row[2].clone();
            let name_b = row[3].clone();

            user_matches_user(world, role_a, name_a, role_b, name_b).await?;
        }
    }

    Ok(())
}

#[then(expr = "{word} {word} matches {word} {word}")]
#[instrument(err(Debug), skip(world))]
async fn user_matches_user(
    world: &mut MarketWorld,
    user_a_role: String,
    user_a_name: String,
    user_b_role: String,
    user_b_name: String,
) -> Result<(), Report<CucumberError>> {
    let user_a_id = match user_a_role.as_str() {
        "buyer" => world.buyers.get(&user_a_name).ok_or(CucumberError::Error)?,
        "seller" => world
            .sellers
            .get(&user_a_name)
            .ok_or(CucumberError::Error)?,
        _ => Err(CucumberError::Error)?,
    };

    let user_b_id = match user_b_role.as_str() {
        "buyer" => world.buyers.get(&user_b_name).ok_or(CucumberError::Error)?,
        "seller" => world
            .sellers
            .get(&user_b_name)
            .ok_or(CucumberError::Error)?,
        _ => Err(CucumberError::Error)?,
    };

    let candidates_a = user::get_candidates(world.pool.as_ref().unwrap().clone(), *user_a_id)
        .await
        .change_context(CucumberError::Error)?;

    debug!("user a candidates: {candidates_a:?}");

    let candidates_b = user::get_candidates(world.pool.as_ref().unwrap().clone(), *user_b_id)
        .await
        .change_context(CucumberError::Error)?;

    debug!("user b candidates: {candidates_b:?}");

    assert_eq!(candidates_a[0].id, candidates_b[0].id);

    Ok(())
}

#[then(expr = "{word} {word} has {int} candidates")]
#[instrument(err(Debug))]
async fn user_has_candidates(
    world: &mut MarketWorld,
    user_role: String,
    user_name: String,
    num_candidates: usize,
) -> Result<(), Report<CucumberError>> {
    /*
        info!("entering user_has_candidates");

        let context = ContextV7::new();
        let timestamp = Timestamp::now(context);

        let mut t = world
            .pool
            .as_ref()
            .unwrap()
            .acquire()
            .await
            .change_context(CucumberError::TransactionError)?;

        // let mut repo = Repository::new(&mut t).await;

        let user_id = match user_role.as_str() {
            "buyer" => world.buyers.get(&user_name).ok_or(CucumberError::Error)?,
            "seller" => world.sellers.get(&user_name).ok_or(CucumberError::Error)?,
            _ => Err(CucumberError::Error)?,
        };

        let user = repo
            .find_user(user_id)
            .await
            .change_context(CucumberError::Error)?;

        debug!("{user:?}");

        let candidates = (*t)
            .find_candidates_by_user(&user)
            .await
            .change_context(CucumberError::Error)?;

        assert_eq!(candidates.len(), num_candidates);
    */
    Ok(())
}
