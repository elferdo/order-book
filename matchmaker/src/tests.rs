use error_stack::Report;
use rstest::rstest;
use thiserror::Error;
use tracing::instrument;

use crate::Market;

#[rstest]
#[instrument(err)]
fn hola() -> Result<(), Report<TestError>> {
    let _market = Market::new();

    // let sell_price = market.sell_price();

    // assert!(sell_price.is_none());

    Ok(())
}

#[derive(Debug, Error)]
#[error("error in test")]
pub struct TestError;
