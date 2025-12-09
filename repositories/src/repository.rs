use model::{order::Order, repository::OrderRepositoryError};
use sqlx::{PgConnection, QueryBuilder};
use tracing::instrument;

pub struct Repository<'c> {
    pub(crate) conn: &'c mut PgConnection,
}

impl<'c> Repository<'c> {
    pub async fn new(conn: &'c mut PgConnection) -> Self {
        Self { conn }
    }

    #[instrument(skip(self))]
    pub async fn persist_order(&mut self, order: &Order) -> Result<(), OrderRepositoryError> {
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

        dbg!(result);

        Ok(())
    }
}
