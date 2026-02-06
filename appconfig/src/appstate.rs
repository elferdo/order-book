use crate::config::Config;
use error_stack::{IntoReport, Report, ResultExt};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
// use thiserror::Error;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    async fn new_pool(config: &Config) -> Result<Pool<Postgres>, Report<Error>> {
        let url = &config.database_url;

        let pool = PgPoolOptions::new()
            .connect(url)
            .await
            .change_context(Error::PoolError)?;

        Ok(pool)
    }

    pub async fn new(config: &Config) -> Result<Self, Report<Error>> {
        let pool = Self::new_pool(config).await?;

        Ok(AppState { pool })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error creating database connection pool for application state")]
    PoolError,
}
