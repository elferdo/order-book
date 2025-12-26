use model::user::repository::UserRepository;
use model::{lock_mode::LockMode, match_service::generate_candidates_for_bid};
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
pub async fn new_bid(pool: PgPool, user_id: Uuid, price: f32) -> Result<Response, BusinessError> {
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

    let bid = user
        .bid(timestamp, price)
        .map_err(|_| BusinessError::UserNotFound)?;

    repo.persist_user(&user)
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    generate_candidates_for_bid(timestamp, &mut repo, &bid)
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    t.commit().await.map_err(|_| BusinessError::DatabaseError)?;

    Ok(Response { id: *bid.get_id() })
}
