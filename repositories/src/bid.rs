use model::{
    lock_mode::LockMode,
    order::Order,
    repository::{BidRepository, OrderRepositoryError},
};
use sqlx::{QueryBuilder, Row, query};
use uuid::Uuid;

use crate::Repository;

impl<'c> BidRepository for Repository<'c> {
    async fn find_bids_above(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> Result<Vec<Order>, OrderRepositoryError> {
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
            .map_err(|_| OrderRepositoryError::DatabaseError)?;

        let bids: Vec<_> = bid_rows
            .into_iter()
            .map(|r| Order::bid_with(r.get("id"), r.get("user"), r.get("price")))
            .collect();

        Ok(bids)
    }

    async fn find_bid(&mut self, id: &Uuid) -> Result<Order, OrderRepositoryError> {
        let bid = query!("select * from bid where id = $1", id)
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| OrderRepositoryError::DatabaseError)?;

        Ok(Order::new_bid(bid.user, bid.price))
    }
}
