use std::sync::Arc;

use model::user::User;
use sqlx::{Postgres, Transaction, query};
use thiserror::Error;
use uuid::Uuid;

pub struct Repository<'a> {
    transaction: Arc<Transaction<'a, Postgres>>,
}

impl<'a> Repository<'a> {
    pub fn new(transaction: Arc<Transaction<'a, Postgres>>) -> Self {
        Self { transaction }
    }

    pub async fn get_user(&mut self, id: &Uuid) -> Result<User, RepositoryError> {
        let user = query!("select * from public.user where id = $1", id)
            .fetch_one(&mut **Arc::get_mut(&mut self.transaction).unwrap())
            .await
            .map_err(|_| RepositoryError::UserNotFound)?;

        Ok(User::new_as(user.id))
    }

    pub async fn persist_user(&mut self, user: &User) -> Result<(), RepositoryError> {
        query!("INSERT INTO public.user (id) VALUES ($1)", user.get_id())
            .execute(&mut **Arc::get_mut(&mut self.transaction).unwrap())
            .await?;

        Ok(())
    }

    pub async fn delete_user(&mut self, user: &User) -> Result<(), RepositoryError> {
        let result = query!("DELETE FROM public.user where id = $1", user.get_id())
            .execute(&mut **Arc::get_mut(&mut self.transaction).unwrap())
            .await?;

        if result.rows_affected() < 1 {
            Err(RepositoryError::OperationFailed)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("user not found")]
    UserNotFound,

    #[error("operation failed")]
    OperationFailed,
}
