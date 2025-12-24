use model::{lock_mode::LockMode, user::user::User};
use model::{repository_error::RepositoryError, user::repository::UserRepository};
use repositories::Repository;
use sqlx::PgPool;
use tracing::{debug, instrument};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::businesserror::BusinessError;
use crate::response::Response;

#[instrument(skip(pool))]
pub async fn new_user(pool: PgPool) -> Result<Response, BusinessError> {
    let mut a = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut a).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let user = User::new(timestamp);

    repo.persist_user(&user).await.map_err(|e| match e {
        RepositoryError::DatabaseError(_) => BusinessError::DatabaseError,
        RepositoryError::UnexpectedResult => todo!(),
        RepositoryError::RootEntityNotFound => todo!(),
    })?;

    Ok(Response {})
}

#[instrument(skip(pool))]
pub async fn delete_user(pool: PgPool, id: Uuid) -> Result<Response, BusinessError> {
    let mut a = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut a).await;

    let user = repo
        .find_user(LockMode::None, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::DatabaseError(_) => BusinessError::DatabaseError,
            RepositoryError::UnexpectedResult => todo!(),
            RepositoryError::RootEntityNotFound => todo!(),
        })?;

    repo.delete_user(&user).await.map_err(|e| match e {
        RepositoryError::DatabaseError(_) => BusinessError::DatabaseError,
        RepositoryError::UnexpectedResult => todo!(),
        RepositoryError::RootEntityNotFound => todo!(),
    })?;

    Ok(Response {})
}
