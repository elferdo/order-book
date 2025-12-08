use model::user::User;
use sqlx::{Postgres, Transaction, query};
use thiserror::Error;
use uuid::Uuid;

pub async fn get_user<'a>(
    transaction: &mut Transaction<'a, Postgres>,
    id: &Uuid,
) -> Result<User, RepositoryError> {
    let user = query!("select * from public.user where id = $1", id)
        .fetch_one(&mut **transaction)
        .await
        .map_err(|_| RepositoryError::UserNotFound)?;

    Ok(User::new_as(user.id))
}

pub async fn persist_user<'a>(
    transaction: &mut Transaction<'a, Postgres>,
    user: &User,
) -> Result<(), RepositoryError> {
    query!("INSERT INTO public.user (id) VALUES ($1)", user.get_id())
        .execute(&mut **transaction)
        .await?;

    Ok(())
}

pub async fn delete_user<'a>(
    transaction: &mut Transaction<'a, Postgres>,
    user: &User,
) -> Result<(), RepositoryError> {
    let result = query!("DELETE FROM public.user where id = $1", user.get_id())
        .execute(&mut **transaction)
        .await?;

    if result.rows_affected() < 1 {
        Err(RepositoryError::OperationFailed)
    } else {
        Ok(())
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
