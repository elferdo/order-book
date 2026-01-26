use error_stack::{Report, ResultExt};
use matchmaker::deal::Deal;
use model::repository_error::RepositoryError;
use sqlx::{PgConnection, query};

use crate::{deal_repository::DealRepository, user::User};

impl DealRepository for PgConnection {
    async fn persist_deal(&mut self, _deal: &Deal) -> Result<(), Report<RepositoryError>> {
        query!(
            "INSERT INTO deal (id, buyer, seller, price) VALUES ($1, $2, $3, $4);",
            *_deal.get_id(),
            *_deal.get_buyer_id(),
            *_deal.get_seller_id(),
            _deal.get_price()
        )
        .execute(self)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

        Ok(())
    }

    async fn find_deals_by_user(
        &mut self,
        user: &User,
    ) -> Result<Vec<Deal>, Report<RepositoryError>> {
        let deal_rows = query!(
            "SELECT * FROM deal WHERE buyer = $1 OR seller = $1;",
            user.get_id()
        )
        .fetch_all(self)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

        let deals = deal_rows
            .iter()
            .map(|row| Deal::with(row.id, row.buyer, row.seller, row.price))
            .collect();

        Ok(deals)
    }
}
