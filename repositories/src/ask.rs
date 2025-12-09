use model::{
    lock_mode::LockMode,
    order::Order,
    repository::{AskRepository, OrderRepositoryError},
};
use sqlx::{QueryBuilder, Row, query};
use uuid::Uuid;

use crate::Repository;

impl<'c> AskRepository for Repository<'c> {
    async fn find_asks_below(
        &mut self,
        lock_mode: LockMode,
        price: f32,
    ) -> Result<Vec<Order>, OrderRepositoryError> {
        let mut qb = QueryBuilder::new("SELECT * FROM ask WHERE price <= $1");
        qb.push_bind(price);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShare => {
                qb.push(" FOR KEY SHARE;");
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

        Ok(Order::new_ask(ask.user, ask.price))
    }
}
