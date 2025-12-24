use model::stats::repository::StatsRepository;
use repositories::Repository;
use sqlx::PgPool;
use tracing::instrument;

use crate::{businesserror::BusinessError, response::Response};

#[instrument(skip(pool))]
pub async fn get_buy_price(pool: PgPool) -> Result<Response, BusinessError> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let _buy_price = repo
        .buy_price()
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    Ok(Response {})
}

#[instrument(skip(pool))]
pub async fn get_sell_price(pool: PgPool) -> Result<Response, BusinessError> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let _sell_price = repo
        .sell_price()
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    Ok(Response {})
}
