use error_stack::{IntoReport, Report};
use rstest::rstest;
use thiserror::Error;
use tracing::{info, instrument};

#[rstest]
#[instrument(err)]
fn hola() -> Result<(), Report<TestError>> {
    info!("hola");

    // assert!(false);

    Err(TestError {}.into_report())
}

#[derive(Debug, Error)]
#[error("error in test")]
struct TestError;
