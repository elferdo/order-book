use model::bid::Bid;
use sqlx::{Postgres, Transaction, query};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

pub struct Repository<'a> {
    transaction: Arc<Transaction<'a, Postgres>>,
}

impl<'a> Repository<'a> {
    pub fn new(transaction: Arc<Transaction<'a, Postgres>>) -> Self {
        Self { transaction }
    }

    pub async fn get_bid(&mut self, id: &Uuid) -> Result<Bid, RepositoryError> {
        let bid = query!("select * from bid where id = $1", id)
            .fetch_one(&mut **Arc::get_mut(&mut self.transaction).unwrap())
            .await?;

        let mut user_repository = super::user::Repository::new(self.transaction.clone());

        let user = user_repository.get_user(&bid.user).await?;

        Ok(Bid::new(Arc::new(user), bid.price))
    }

    pub async fn persist_bid(&mut self, bid: &Bid) -> Result<(), RepositoryError> {
        query!(
            "INSERT INTO bid VALUES ($1, $2, $3)",
            bid.get_id(),
            bid.get_user().get_id(),
            bid.get_price()
        )
        .execute(&mut **Arc::get_mut(&mut self.transaction).unwrap())
        .await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("user error")]
    UserError(#[from] super::user::RepositoryError),
}
