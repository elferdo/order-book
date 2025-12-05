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
        let user = query!("select * from public.user where id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| RepositoryError::UserNotFound)?;

        Ok(User::new_as(user.id))
    }

    pub async fn persist_user(&self, user: &User) -> Result<(), RepositoryError> {
        query!("INSERT INTO public.user (id) VALUES ($1)", user.get_id())
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("user not found")]
    UserNotFound,
}
