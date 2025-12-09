use model::{
    bid::Bid,
    lock_mode::LockMode,
    repository::{BidRepository, BidRepositoryError},
};
use sqlx::{QueryBuilder, Row, query};
use uuid::Uuid;

use crate::Repository;

impl<'c> BidRepository for Repository<'c> {
    async fn find_bids_above(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> Result<Vec<Bid>, BidRepositoryError> {
        let mut qb = QueryBuilder::new("SELECT * FROM bid WHERE price <= $1");
        qb.push_bind(price);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShare => {
                qb.push(" FOR KEY SHARE;");
            }
        };

        let bid_rows = qb
            .build()
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|_| BidRepositoryError::DatabaseError)?;

        let bids: Vec<_> = bid_rows
            .into_iter()
            .map(|r| Bid::with(r.get("id"), r.get("user"), r.get("price")))
            .collect();

        Ok(bids)
    }

    async fn find_bid(&mut self, id: &Uuid) -> Result<Bid, BidRepositoryError> {
        let bid = query!("select * from bid where id = $1", id)
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| BidRepositoryError::DatabaseError)?;

        Ok(Bid::new(bid.user, bid.price))
    }

    async fn persist_bid(&mut self, bid: &Bid) -> Result<(), BidRepositoryError> {
        query!(
            "INSERT INTO bid VALUES ($1, $2, $3)",
            bid.get_id(),
            bid.get_user_id(),
            bid.get_price()
        )
        .execute(&mut *self.conn)
        .await
        .map_err(|_| BidRepositoryError::DatabaseError)?;

        Ok(())
    }
}
