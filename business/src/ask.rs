use crate::businesserror::BusinessError;
use error_stack::IntoReport;
use error_stack::Report;
use error_stack::ResultExt;
use model::match_service::generate_candidates_for_ask;
use model::user::repository::UserRepository;
use repositories::Repository;
use retry::OperationResult;
use retry::delay::Fixed;
use retry::retry;
use serde::Serialize;
use sqlx::PgPool;
use sqlx::query;
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Serialize)]
pub struct Response {
    pub id: Uuid,
}

#[instrument(skip(pool))]
pub async fn new_ask(
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

    let ask = user.ask(timestamp, price);

    repo.persist_user(&user)
        .await
        .change_context(BusinessError::UserPersistenceError)?;

    t.commit()
        .await
        .change_context(BusinessError::DatabaseError)?;

    let mut t2 = pool
        .begin()
        .await
        .change_context(BusinessError::DatabaseError)?;

    query!("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE;")
        .execute(&mut *t2)
        .await
        .unwrap();

    let mut repo2 = Repository::new(&mut t2).await;

    let mut success = false;

    for _ in 0..2 {
        match generate_candidates_for_ask(timestamp, &mut repo2, &ask).await {
            Ok(_) => {
                success = true;
                break;
            }
            Err(_) => continue,
        }
    }

    if !success {
        return Err(BusinessError::DatabaseError.into_report());
    }

    t2.commit()
        .await
        .change_context(BusinessError::DatabaseError)?;

    Ok(Response { id: *ask.get_id() })
}
