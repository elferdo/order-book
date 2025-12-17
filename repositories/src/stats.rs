use model::repository_error::RepositoryError;
use model::stats::repository::StatsRepository;
use sqlx::query;

use crate::Repository;

impl<'c> StatsRepository for Repository<'c> {
    async fn buy_price(&mut self) -> Result<f32, RepositoryError> {
        let row = query!("SELECT MAX(price) AS price FROM bid LEFT JOIN candidate ON candidate.bid = bid.id WHERE candidate.ask IS NULL;")
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| RepositoryError::DatabaseError)?;

        if let Some(price) = row.price {
            Ok(price)
        } else {
            Err(RepositoryError::UnexpectedResult)
        }
    }

    async fn sell_price(&mut self) -> Result<f32, RepositoryError> {
        let row = query!("SELECT MIN(price) AS price FROM ask LEFT JOIN candidate ON candidate.ask = ask.id WHERE candidate.bid IS NULL;")
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| RepositoryError::DatabaseError)?;

        if let Some(price) = row.price {
            Ok(price)
        } else {
            Err(RepositoryError::UnexpectedResult)
        }
    }
}
