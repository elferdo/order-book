use model::repository_error::RepositoryError;
use model::{
    deal::{Deal, repository::DealRepository},
    user::user::User,
};
use sqlx::query;

use crate::Repository;

impl<'c> DealRepository for Repository<'c> {
    async fn persist_deal(&mut self, _deal: &model::deal::Deal) -> Result<(), RepositoryError> {
        query!(
            "INSERT INTO deal (id, buyer, seller, price) VALUES ($1, $2, $3, $4);",
            *_deal.get_id(),
            *_deal.get_buyer_id(),
            *_deal.get_seller_id(),
            _deal.get_price()
        )
        .execute(&mut *self.conn)
        .await?;

        todo!()
    }

    async fn find_deals_by_user(
        &mut self,
        user: &User,
    ) -> Result<Vec<model::deal::Deal>, RepositoryError> {
        let deal_rows = query!(
            "SELECT * FROM deal WHERE buyer = $1 OR seller = $1;",
            user.get_id()
        )
        .fetch_all(&mut *self.conn)
        .await?;

        let deals = deal_rows
            .iter()
            .map(|row| Deal::with(row.id, row.buyer, row.seller, row.price))
            .collect();

        Ok(deals)
    }
}
