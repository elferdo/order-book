use model::{
    ask::Ask,
    repository::{AskRepository, AskRepositoryError},
};
use sqlx::query;
use uuid::Uuid;

use crate::Repository;

impl<'c> AskRepository for Repository<'c> {
    async fn find_asks_below(&mut self, price: f32) -> Result<Vec<Ask>, AskRepositoryError> {
        let ask_rows = query!("select * from ask where price <= $1", price)
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|_| AskRepositoryError::DatabaseError)?;

        let asks: Vec<_> = ask_rows
            .into_iter()
            .map(|r| Ask::with(r.id, r.user, r.price))
            .collect();

        Ok(asks)
    }

    async fn find_ask(&mut self, id: &Uuid) -> Result<Ask, AskRepositoryError> {
        let ask = query!("select * from ask where id = $1", id)
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| AskRepositoryError::DatabaseError)?;

        Ok(Ask::new(ask.user, ask.price))
    }

    async fn persist_ask(&mut self, ask: &Ask) -> Result<(), AskRepositoryError> {
        query!(
            "INSERT INTO ask VALUES ($1, $2, $3)",
            ask.get_id(),
            ask.get_user_id(),
            ask.get_price()
        )
        .execute(&mut *self.conn)
        .await
        .map_err(|_| AskRepositoryError::DatabaseError)?;

        Ok(())
    }
}
