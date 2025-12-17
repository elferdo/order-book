use model::lock_mode::LockMode;
use model::order::ask::Ask;
use model::order::bid::Bid;
use model::order::order::Order;
use model::order::repository::{OrderRepository, OrderRepositoryError};
use sqlx::{Database, Postgres, QueryBuilder};
use sqlx::{Row, query};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::Repository;

impl<'c> OrderRepository for Repository<'c> {
    async fn find_asks_below(
        &mut self,
        lock_mode: LockMode,
        bid: &Bid,
    ) -> Result<Vec<Ask>, OrderRepositoryError> {
        let mut qb = QueryBuilder::new("SELECT ask.id, ask.user, ask.price FROM ask LEFT JOIN candidate ON candidate.ask = ask.id WHERE candidate.bid IS NULL AND
 price <= ");
        qb.push_bind(bid.get_price());
        qb.push(" AND ask.user <> ");
        qb.push_bind(bid.get_user_id());

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
            .map(|r| Ask::with(r.get("id"), r.get("user"), r.get("price")))
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
        ask: &Ask,
    ) -> Result<Vec<Bid>, OrderRepositoryError> {
        let mut qb = QueryBuilder::new("SELECT bid.id, bid.user, bid.price FROM bid LEFT JOIN candidate ON candidate.bid = bid.id WHERE candidate.bid IS NULL AND
 price >= ");
        qb.push_bind(ask.get_price());
        qb.push(" AND bid.user <> ");
        qb.push_bind(ask.get_user_id());

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
            .map(|r| Bid::with(r.get("id"), r.get("user"), r.get("price")))
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

    fn persist_ask(&mut self, ask: &Ask) -> impl Future<Output = Result<(), OrderRepositoryError>> {
        let order = Order::from(ask);

        persist_order(&mut *self.conn, order)
    }

    #[instrument(skip(self))]
    fn persist_bid(&mut self, bid: &Bid) -> impl Future<Output = Result<(), OrderRepositoryError>> {
        debug!("entering persist_bid()");

        let order = Order::from(bid);

        persist_order(&mut *self.conn, order)
    }

    async fn remove_ask(&mut self, ask: &Ask) -> Result<(), OrderRepositoryError> {
        query!("DELETE FROM ask WHERE id = $1;", *ask.get_id())
            .execute(&mut *self.conn)
            .await
            .map_err(|_| OrderRepositoryError::DatabaseError)?;

        Ok(())
    }

    async fn remove_bid(&mut self, bid: &Bid) -> Result<(), OrderRepositoryError> {
        query!("DELETE FROM bid WHERE id = $1;", *bid.get_id())
            .execute(&mut *self.conn)
            .await
            .map_err(|_| OrderRepositoryError::DatabaseError)?;

        Ok(())
    }
}

#[instrument]
async fn persist_order(
    conn: &mut <Postgres as Database>::Connection,
    order: Order,
) -> Result<(), OrderRepositoryError> {
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
        .execute(&mut *conn)
        .await
        .map_err(|_| OrderRepositoryError::DatabaseError)?;

    if result.rows_affected() < 1 {
        Err(OrderRepositoryError::DatabaseError)
    } else {
        Ok(())
    }
}
