use crate::config::Config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
// use thiserror::Error;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    async fn new_pool(config: &Config) -> Result<Pool<Postgres>, Error> {
        let url = &config.database_url;

        let pool = PgPoolOptions::new().connect(url).await?;

        Ok(pool)
    }

    pub async fn new(config: &Config) -> Result<Self, Error> {
        let pool = Self::new_pool(config).await?;

        Ok(AppState { pool })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("")]
    PoolError(#[from] sqlx::Error),
}
