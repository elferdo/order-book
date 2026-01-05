use model::stats::repository::StatsRepository;
use repositories::Repository;
use serde::Serialize;
use sqlx::PgPool;
use tracing::instrument;

use crate::businesserror::BusinessError;

#[derive(Serialize)]
pub struct SellResponse {
    sell_price: f32,
}

#[derive(Serialize)]
pub struct BuyResponse {
    buy_price: f32,
}

#[instrument(skip(pool))]
pub async fn get_buy_price(pool: PgPool) -> Result<BuyResponse, BusinessError> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let buy_price = repo
        .buy_price()
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    Ok(BuyResponse { buy_price })
}

#[instrument(skip(pool))]
pub async fn get_sell_price(pool: PgPool) -> Result<SellResponse, BusinessError> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let sell_price = repo
        .sell_price()
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    Ok(SellResponse { sell_price })
}
