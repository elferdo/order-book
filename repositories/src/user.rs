use model::user::User;
use sqlx::{Pool, Postgres, query};
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

        let user = query
            .fetch_one(&self.pool)
            .await
            .map_err(|_| RepositoryError::UserNotFound)?;

        Ok(User::new_from(user.id))
    }

    pub async fn persist_user(&self, user: &User) -> Result<(), RepositoryError> {
        let query = query!("INSERT INTO public.user (id) VALUES ($1)", user.get_id());

        // query_builder.push(" ON CONFLICT (d) DO UPDATE SET id = EXCLUDED.id");

        query.execute(&self.pool).await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository error")]
    Error(#[from] sqlx::Error),

    #[error("user not found")]
    UserNotFound,
}
