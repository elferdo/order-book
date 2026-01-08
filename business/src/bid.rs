use crate::businesserror::BusinessError;
use error_stack::Report;
use error_stack::ResultExt;
use model::match_service::generate_candidates_for_bid;
use model::user::repository::UserRepository;
use repositories::Repository;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::query;
use tracing::debug;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize)]
pub struct Response {
    pub id: Uuid,
}

#[instrument(skip(pool))]
pub async fn new_bid(
    pool: PgPool,
    user_id: Uuid,
    price: f32,
) -> Result<Response, Report<BusinessError>> {
    let mut t = pool
        .begin()
        .await
        .change_context(BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut t).await;

    let mut user = repo
        .find_user(&user_id)
        .await
        .change_context(BusinessError::UserNotFound)?;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let bid = user.bid(timestamp, price);

    repo.persist_user(&user)
        .await
        .change_context(BusinessError::UserPersistenceError)?;

    t.commit()
        .await
        .change_context(BusinessError::DatabaseError)?;

    let mut t = pool
        .begin()
        .await
        .change_context(BusinessError::DatabaseError)?;

    query!("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;")
        .execute(&mut *t)
        .await
        .unwrap();

    let mut repo = Repository::new(&mut t).await;

    generate_candidates_for_bid(timestamp, &mut repo, &bid)
        .await
        .change_context(BusinessError::MatchingError)?;

    t.commit()
        .await
        .change_context(BusinessError::DatabaseError)?;

    Ok(Response { id: *bid.get_id() })
}
