use model::{
    order_match::Match,
    repository::{OrderMatchRepository, OrderMatchRepositoryError},
};
use sqlx::{QueryBuilder, query};
use uuid::Uuid;

use crate::Repository;

impl<'c> OrderMatchRepository for Repository<'c> {
    async fn persist_order_matches<I>(
        &mut self,
        iterator: I,
    ) -> Result<(), OrderMatchRepositoryError>
    where
        I: IntoIterator<Item = Match>,
    {
        let mut peekable = iterator.into_iter().peekable();

        if peekable.next().is_none() {
            return Ok(());
        };

        let mut qb = QueryBuilder::new("INSERT INTO match ");

        qb.push_values(peekable, |mut b, m| {
            dbg!(&m);

            b.push_bind(*m.get_ask()).push_bind(*m.get_bid());
        });

        let _ = qb
            .build()
            .execute(&mut *self.conn)
            .await
            .map_err(|_| OrderMatchRepositoryError::DatabaseError);

        Ok(())
    }

    async fn get_order_match(
        &mut self,
        ask: &Uuid,
        bid: &Uuid,
    ) -> Result<Match, OrderMatchRepositoryError> {
        let order_match = query!("select * from match where ask = $1 and bid = $2", ask, bid)
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| OrderMatchRepositoryError::DatabaseError)?;

        Ok(Match::new(order_match.ask, order_match.bid))
    }

    async fn persist_order_match(
        &mut self,
        order_match: &Match,
    ) -> Result<(), OrderMatchRepositoryError> {
        query!(
            "INSERT INTO match VALUES ($1, $2)",
            order_match.get_ask(),
            order_match.get_bid(),
        )
        .execute(&mut *self.conn)
        .await
        .map_err(|_| OrderMatchRepositoryError::DatabaseError)?;

        Ok(())
    }
}
