use model::{
    lock_mode::LockMode,
    order::Order,
    repository::{OrderRepository, OrderRepositoryError},
};
use sqlx::QueryBuilder;
use sqlx::{Row, query};
use tracing::instrument;
use uuid::Uuid;

use crate::Repository;

impl<'c> OrderRepository for Repository<'c> {
    async fn find_asks_below(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> Result<Vec<Order>, OrderRepositoryError> {
        let mut qb = QueryBuilder::new("SELECT ask.id, ask.user, ask.price FROM ask LEFT JOIN match ON match.ask = ask.id WHERE match.bid IS NULL AND
 price <= ");
        qb.push_bind(price);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShare => {
                qb.push(" FOR KEY SHARE OF ask;");
            }
        };

        let ask_rows = qb
            .build()
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|_| OrderRepositoryError::DatabaseError)?;

        let asks: Vec<_> = ask_rows
            .into_iter()
            .map(|r| Order::ask_with(r.get("id"), r.get("user"), r.get("price")))
            .collect();

        Ok(asks)
    }

    async fn find_ask(&mut self, id: &Uuid) -> Result<Order, OrderRepositoryError> {
        let ask = query!("select * from ask where id = $1", id)
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| OrderRepositoryError::DatabaseError)?;

        Ok(Order::ask_with(ask.id, ask.user, ask.price))
    }

    async fn find_bids_above(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> Result<Vec<Order>, OrderRepositoryError> {
        let mut qb = QueryBuilder::new("SELECT bid.id, bid.user, bid.price FROM bid LEFT JOIN match ON match.bid = bid.id WHERE match.bid IS NULL AND
 price >= ");
        qb.push_bind(price);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShare => {
                qb.push(" FOR KEY SHARE OF bid;");
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

        Ok(Order::bid_with(bid.id, bid.user, bid.price))
    }

    #[instrument(skip(self))]
    async fn persist_order(&mut self, order: &Order) -> Result<(), OrderRepositoryError> {
        let mut qb = QueryBuilder::new("INSERT INTO ");

        let table_name = match order {
            Order::Ask { .. } => "ask",
            Order::Bid { .. } => "bid",
        };

        qb.push(table_name);
        qb.push(" ");

        qb.push_values([order], |mut b, o| {
            b.push_bind(*o.get_id())
                .push_bind(*o.get_user_id())
                .push_bind(o.get_price());
        });

        let query = qb.build();

        let result = query
            .execute(&mut *self.conn)
            .await
            .map_err(|_| OrderRepositoryError::DatabaseError)?;

        if result.rows_affected() < 1 {
            Err(OrderRepositoryError::DatabaseError)
        } else {
            Ok(())
        }
    }
}
