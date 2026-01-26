use error_stack::{Report, ResultExt};
use sqlx::{PgConnection, query};

use crate::{repository::StatsRepository, repository_error::RepositoryError};

impl StatsRepository for PgConnection {
    async fn buy_price(&mut self) -> Result<f32, Report<RepositoryError>> {
        let row = query!("SELECT MAX(not_above) AS price FROM bid LEFT JOIN candidate ON candidate.bid = bid.id WHERE candidate.ask IS NULL;")
            .fetch_one(self)
            .await.change_context( RepositoryError::UnexpectedResult)?;

        if let Some(price) = row.price {
            Ok(price)
        } else {
            Err(Report::new(RepositoryError::UnexpectedResult))
        }
    }

    async fn sell_price(&mut self) -> Result<f32, Report<RepositoryError>> {
        let row = query!("SELECT MIN(not_below) AS price FROM ask LEFT JOIN candidate ON candidate.ask = ask.id WHERE candidate.bid IS NULL;")
            .fetch_one(self)
            .await.change_context(
            RepositoryError::UnexpectedResult)?;

        if let Some(price) = row.price {
            Ok(price)
        } else {
            Err(Report::new(RepositoryError::UnexpectedResult))
        }
    }
}
