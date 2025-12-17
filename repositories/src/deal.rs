use model::{
    deal::{
        Deal,
        repository::{DealRepository, DealRepositoryError},
    },
    user::user::User,
};
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

    async fn find_deals_by_user(
        &mut self,
        user: &User,
    ) -> Result<Vec<model::deal::Deal>, DealRepositoryError> {
        let deal_rows = query!(
            "SELECT * FROM deal WHERE buyer = $1 OR seller = $1;",
            user.get_id()
        )
        .fetch_all(&mut *self.conn)
        .await
        .map_err(|_| DealRepositoryError::DatabaseError)?;

        let deals = deal_rows
            .iter()
            .map(|row| Deal::with(row.id, row.buyer, row.seller, row.price))
            .collect();

        Ok(deals)
    }
}
