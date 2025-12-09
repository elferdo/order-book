use model::{order::Order, repository::OrderRepositoryError};
use sqlx::{PgConnection, QueryBuilder};

pub struct Repository<'c> {
    pub(crate) conn: &'c mut PgConnection,
}

impl<'c> Repository<'c> {
    pub async fn new(conn: &'c mut PgConnection) -> Self {
        Self { conn }
    }

    pub async fn persist_order(&mut self, order: &Order) -> Result<(), OrderRepositoryError> {
        let mut qb = QueryBuilder::new("INSERT INTO ");

        let table_name = match order {
            Order::Ask { .. } => "ask",
            Order::Bid { .. } => "bid",
        };

        qb.push(table_name);

        qb.push_bind(order.get_id())
            .push_bind(order.get_user_id())
            .push_bind(order.get_price())
            .build()
            .execute(&mut *self.conn)
            .await
            .map_err(|_| OrderRepositoryError::DatabaseError)?;

        Ok(())
    }
}
