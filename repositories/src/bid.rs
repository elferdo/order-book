use model::{
    bid::Bid,
    repository::{BidRepository, BidRepositoryError},
};
use sqlx::query;
use uuid::Uuid;

use crate::Repository;

impl<'c> BidRepository for Repository<'c> {
    async fn find_bids_below(&mut self, price: f32) -> Result<Vec<Bid>, BidRepositoryError> {
        todo!()
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
