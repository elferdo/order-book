use model::deal::repository::DealRepository;
use model::lock_mode::LockMode;
use model::user::repository::UserRepository;
use repositories::Repository;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::businesserror::BusinessError;
use crate::response::Response;

#[instrument(skip(pool))]
pub async fn get_deals(pool: PgPool, user_id: Uuid) -> Result<Response, BusinessError> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let user = repo
        .find_user(LockMode::KeyShare, &user_id)
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    let _deals = repo
        .find_deals_by_user(&user)
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    Ok(Response {})
}
