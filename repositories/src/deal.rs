use model::deal::repository::{DealRepository, DealRepositoryError};
use sqlx::query;

use crate::Repository;

impl<'c> DealRepository for Repository<'c> {
    async fn persist_deal(&mut self, _deal: &model::deal::Deal) -> Result<(), DealRepositoryError> {
        query!(
            "INSERT INTO deal (id, buyer, seller, price) VALUES ($1, $2, $3, $4);",
            *_deal.get_id(),
            *_deal.get_buyer_id(),
            *_deal.get_seller_id(),
            _deal.get_price()
        )
        .execute(&mut *self.conn)
        .await
        .map_err(|_| DealRepositoryError::Error)?;

        todo!()
    }
}
