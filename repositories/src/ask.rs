use model::{
    ask::Ask,
    lock_mode::LockMode,
    repository::{AskRepository, AskRepositoryError},
};
use sqlx::{QueryBuilder, Row, query};
use uuid::Uuid;

use crate::Repository;

impl<'c> AskRepository for Repository<'c> {
    async fn find_asks_below(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> Result<Vec<Ask>, AskRepositoryError> {
        let mut qb = QueryBuilder::new("SELECT * FROM ask WHERE price <= $1");
        qb.push_bind(price);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShare => {
                qb.push(" FOR KEY SHARE;");
            }
        };

        let ask_rows = qb
            .build()
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|_| AskRepositoryError::DatabaseError)?;

        let asks: Vec<_> = ask_rows
            .into_iter()
            .map(|r| Ask::with(r.get("id"), r.get("user"), r.get("price")))
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
