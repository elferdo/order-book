use model::stats::repository::{StatsRepository, StatsRepositoryError};
use sqlx::query;

use crate::Repository;

impl<'c> StatsRepository for Repository<'c> {
    async fn buy_price(&mut self) -> Result<f32, model::stats::repository::StatsRepositoryError> {
        let row = query!("SELECT MAX(price) AS price FROM bid LEFT JOIN match ON match.bid = bid.id WHERE match.ask IS NULL;")
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| StatsRepositoryError::DatabaseError)?;

        if let Some(price) = row.price {
            Ok(price)
        } else {
            Err(StatsRepositoryError::DatabaseError)
        }
    }

    async fn sell_price(&mut self) -> Result<f32, StatsRepositoryError> {
        let row = query!("SELECT MIN(price) AS price FROM ask LEFT JOIN match ON match.ask = ask.id WHERE match.bid IS NULL;")
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| StatsRepositoryError::DatabaseError)?;

        if let Some(price) = row.price {
            Ok(price)
        } else {
            Err(StatsRepositoryError::DatabaseError)
        }
    }
}
