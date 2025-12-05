use std::sync::Arc;

use model::bid::Bid;
use sqlx::{Pool, Postgres, QueryBuilder, query};
use thiserror::Error;
use uuid::Uuid;

pub struct Repository {
    pool: Pool<Postgres>,
}

impl Repository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_bid(&self, id: &Uuid) -> Result<Bid, RepositoryError> {
        let query = query!("select * from bid where id = $1", id);

        let bid = query.fetch_one(&self.pool).await?;

        let user_repository = super::user::Repository::new(self.pool.clone());

        let user = user_repository.get_user(&bid.user).await?;

        Ok(Bid::new(Arc::new(user), bid.price))
    }

    pub async fn persist_bid(&self, bid: &Bid) -> Result<(), RepositoryError> {
        let query = query!(
            "INSERT INTO bid VALUES ($1, $2, $3)",
            bid.get_id(),
            bid.get_user().get_id(),
            bid.get_price()
        );

        // query_builder.push(" ON CONFLICT (d) DO UPDATE SET id = EXCLUDED.id");

        query.execute(&self.pool).await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository error")]
    DbError(#[from] sqlx::Error),

    #[error("user error")]
    UserError(#[from] super::user::RepositoryError),
}
