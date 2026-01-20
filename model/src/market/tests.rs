use error_stack::{IntoReport, Report};
use rstest::rstest;
use thiserror::Error;
use tracing::{info, instrument};

use crate::market::Market;

#[rstest]
#[instrument(err)]
fn hola() -> Result<(), Report<TestError>> {
    let market = Market::new();

    let sell_price = market.sell_price();

    assert!(sell_price.is_none());

    Ok(())
}

#[derive(Debug, Error)]
#[error("error in test")]
pub struct TestError;
