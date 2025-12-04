use model::user::User;
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

    pub async fn get_user(&self, id: &Uuid) -> Result<User, RepositoryError> {
        let query = query!("select * from public.user where id = $1", id);

        let user = query.fetch_one(&self.pool).await?;

        Ok(User::new_from(user.id))
    }

    pub async fn persist_user(&self, user: &User) -> Result<(), RepositoryError> {
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
    Error(#[from] sqlx::Error),
}
