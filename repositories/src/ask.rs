use model::{
    ask::Ask,
    match_maker::{self, AskRepositoryError},
};
use sqlx::{Database, Postgres, query};
use thiserror::Error;
use uuid::Uuid;

pub struct AskRepository<'c> {
    // conn: &'c mut <Postgres as Database>::Connection,
    conn: &'c mut <Postgres as Database>::Connection,
}

impl<'c> AskRepository<'c> {
    pub fn new(conn: &'c mut <Postgres as Database>::Connection) -> Self {
        Self { conn }
    }

    pub async fn get_ask(&mut self, id: &Uuid) -> Result<Ask, RepositoryError> {
        let ask = query!("select * from ask where id = $1", id)
            .fetch_one(&mut *self.conn)
            .await?;

        Ok(Ask::new(ask.user, ask.price))
    }

    pub async fn persist_ask(&mut self, ask: &Ask) -> Result<(), RepositoryError> {
        query!(
            "INSERT INTO ask VALUES ($1, $2, $3)",
            ask.get_id(),
            ask.get_user_id(),
            ask.get_price()
        )
        .execute(&mut *self.conn)
        .await?;

        Ok(())
    }
}

impl<'c> match_maker::AskRepository for AskRepository<'c> {
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
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("user error")]
    UserError(#[from] super::user::RepositoryError),
}
