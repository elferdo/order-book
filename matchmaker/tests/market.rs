use std::collections::HashMap;

use error_stack::{Report, ResultExt};
use rstest::{fixture, rstest};
use thiserror::Error;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

use matchmaker::Market;

struct Users {
    pub buyers: HashMap<String, Uuid>,
    pub sellers: HashMap<String, Uuid>,
}

#[fixture]
fn users() -> Users {
    let mut buyers = HashMap::default();
    let mut sellers = HashMap::default();

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    buyers.insert("Barbara".to_string(), Uuid::new_v7(timestamp));
    buyers.insert("Bert".to_string(), Uuid::new_v7(timestamp));
    buyers.insert("Bob".to_string(), Uuid::new_v7(timestamp));

    sellers.insert("Sandra".to_string(), Uuid::new_v7(timestamp));
    sellers.insert("Simon".to_string(), Uuid::new_v7(timestamp));
    sellers.insert("Susan".to_string(), Uuid::new_v7(timestamp));

    Users { buyers, sellers }
}

#[rstest]
#[tokio::test]
#[instrument(err)]
async fn given_empty_market_when_run_then_no_candidates() -> Result<(), Report<TestError>> {
    let asks = Vec::default();
    let bids = Vec::default();

    let mut market = Market::new(asks, bids);

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let result = market.run(timestamp).await.change_context(TestError {})?;

    assert!(result.is_empty());

    Ok(())
}

#[rstest]
#[tokio::test]
#[instrument(err)]
async fn f(users: Users) -> Result<(), Report<TestError>> {
    let asks = Vec::default();
    let bids = Vec::default();

    let mut market = Market::new(asks, bids);

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let result = market.run(timestamp).await.change_context(TestError {})?;

    assert!(result.is_empty());
    assert_eq!(users.buyers["Bob"], users.sellers["Sandra"]);

    Ok(())
}

#[derive(Debug, Error)]
#[error("error in test")]
pub struct TestError;
