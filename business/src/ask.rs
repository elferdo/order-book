use crate::{businesserror::BusinessError, response::Response};
use model::user::repository::UserRepository;
use model::{lock_mode::LockMode, match_service::generate_candidates_for_ask};
use repositories::Repository;
use sqlx::PgPool;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

#[instrument(skip(pool))]
pub async fn new_ask(pool: PgPool, user_id: Uuid, price: f32) -> Result<Response, BusinessError> {
    let mut t = pool
        .begin()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut t).await;

    let mut user = repo
        .find_user(LockMode::KeyShare, &user_id)
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let ask = user
        .ask(timestamp, price)
        .map_err(|_| BusinessError::UserNotFound)?;

    repo.persist_user(&user)
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    generate_candidates_for_ask(timestamp, &mut repo, &ask)
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    t.commit().await.map_err(|_| BusinessError::DatabaseError)?;

    Ok(Response {})
}
