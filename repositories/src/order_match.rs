use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use model::{
    ask::Ask,
    bid::Bid,
    order_match::Match,
    repository::{OrderMatchRepository, OrderMatchRepositoryError},
    user::User,
};
use sqlx::{QueryBuilder, query};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::Repository;

impl<'c> OrderMatchRepository for Repository<'c> {
    #[instrument(skip(self, iterator))]
    async fn persist_order_matches<I>(
        &mut self,
        iterator: I,
    ) -> Result<(), OrderMatchRepositoryError>
    where
        I: IntoIterator<Item = Match>,
    {
        let mut peekable = iterator.into_iter().peekable();

        if peekable.peek().is_none() {
            return Ok(());
        };

        let mut qb = QueryBuilder::new("INSERT INTO match ");

        qb.push_values(peekable, |mut b, m| {
            b.push_bind(*m.get_id())
                .push_bind(*m.get_ask().get_id())
                .push_bind(*m.get_bid().get_id());
        });

        let _ = qb
            .build()
            .execute(&mut *self.conn)
            .await
            .map_err(|_| OrderMatchRepositoryError::DatabaseError);

        Ok(())
    }

    async fn find_order_matches_by_user(
        &mut self,
        user: &User,
    ) -> Result<Vec<Match>, OrderMatchRepositoryError> {
        let order_match_rows = query!("SELECT match.id, match.ask, match.bid, ask.price as ask_price, bid.price as bid_price FROM match JOIN ask ON match.ask = ask.id JOIN bid ON match.bid = bid.id WHERE ask.user = $1 OR bid.user = $1", user.get_id())
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|_| OrderMatchRepositoryError::DatabaseError)?;

        let mut asks = HashMap::new();
        asks = order_match_rows
            .iter()
            .map(|r| (r.ask, Ask::with(r.ask, *user.get_id(), r.ask_price)))
            .collect();

        let mut bids = HashMap::new();
        bids = order_match_rows
            .iter()
            .map(|r| (r.bid, Bid::with(r.bid, *user.get_id(), r.bid_price)))
            .collect();

        let order_matches = order_match_rows
            .iter()
            .map(|r| {
                let ask = Arc::new(asks.remove(&r.ask).unwrap());
                let bid = Arc::new(bids.remove(&r.bid).unwrap());
                Match::with(r.id, ask, bid)
            })
            .collect();

        Ok(order_matches)
    }

    async fn persist_order_match(
        &mut self,
        order_match: &Match,
    ) -> Result<(), OrderMatchRepositoryError> {
        query!(
            "INSERT INTO match VALUES ($1, $2, $3)",
            order_match.get_id(),
            order_match.get_ask().get_id(),
            order_match.get_bid().get_id(),
        )
        .execute(&mut *self.conn)
        .await
        .map_err(|_| OrderMatchRepositoryError::DatabaseError)?;

        Ok(())
    }
}
