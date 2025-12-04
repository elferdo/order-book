use model::bid::Bid;
use sqlx::{Pool, Postgres, QueryBuilder, query};
use std::rc::Rc;
use thiserror::Error;
use uuid::Uuid;

pub struct Repository {
    pool: Pool<Postgres>,
}

impl Repository {
    pub async fn get_bid(&self, id: &Uuid) -> Result<Bid, RepositoryError> {
        let query = query!("select * from bid where id = $1", id);

        let bid = query.fetch_one(&self.pool).await?;

        let user_repository = super::user::Repository::new(self.pool.clone());

        let user = user_repository.get_user(&bid.user).await?;

        Ok(Bid::new(Rc::new(user), bid.price))
    }

    pub async fn persist_user(&self, user: &Bid) -> Result<(), RepositoryError> {
        let mut query_builder = QueryBuilder::new("INSERT INTO user (id) ");

        query_builder.push_bind(user.get_id());

        // query_builder.push(" ON CONFLICT (d) DO UPDATE SET id = EXCLUDED.id");

        let query = query_builder.build();

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
