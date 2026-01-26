use matchmaker::deal::Deal;
use matchmaker::deal_repository::DealRepository;
use serde::Serialize;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::businesserror::BusinessError;
use crate::repository::UserRepository;

#[derive(Serialize)]
pub struct Response {}

#[derive(Serialize)]
pub struct DealSummary {
    pub id: Uuid,
    pub buyer: Uuid,
    pub seller: Uuid,
    pub price: f32,
}

impl From<Deal> for DealSummary {
    fn from(value: Deal) -> Self {
        let id = *value.get_id();
        let buyer = *value.get_buyer_id();
        let seller = *value.get_seller_id();
        let price = value.get_price();

        Self {
            id,
            buyer,
            seller,
            price,
        }
    }
}

#[instrument(skip(pool))]
pub async fn get_deals(pool: PgPool, user_id: Uuid) -> Result<Vec<DealSummary>, BusinessError> {
    let mut conn = pool
        .acquire()
        .await
        .map_err(|_| BusinessError::DatabaseError)?;

    let user = (*conn)
        .find_user(&user_id)
        .await
        .map_err(|_| BusinessError::UserNotFound)?;

    let deals = (*conn)
        .find_deals_by_user(&user.get_id())
        .await
        .map_err(|_| BusinessError::DatabaseError)?
        .into_iter()
        .map(DealSummary::from)
        .collect();

    Ok(deals)
}
