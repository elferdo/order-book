use error_stack::{Report, ResultExt};
use model::user::repository::UserRepository;
use model::{lock_mode::LockMode, user::user::User};
use repositories::Repository;
use serde::Serialize;
use sqlx::PgPool;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::businesserror::BusinessError;

#[derive(Serialize)]
pub struct Response {
    id: Uuid,
}

#[instrument(skip(pool))]
pub async fn new_user(pool: PgPool) -> Result<Response, Report<BusinessError>> {
    let mut a = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut a).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let user = User::new(timestamp);

    repo.persist_user(&user)
        .await
        .change_context(BusinessError::DatabaseError)?;

    Ok(Response { id: *user.get_id() })
}

#[instrument(skip(pool))]
pub async fn delete_user(pool: PgPool, id: Uuid) -> Result<Response, Report<BusinessError>> {
    let mut a = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut a).await;

    let user = repo
        .find_user(LockMode::None, &id)
        .await
        .change_context(BusinessError::DatabaseError)?;

    repo.delete_user(&user)
        .await
        .change_context(BusinessError::DatabaseError)?;

    Ok(Response { id: *user.get_id() })
}
