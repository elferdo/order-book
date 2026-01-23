use error_stack::{Report, ResultExt};
use model::market::repository::MarketRepository;
use model::order::ask::Ask;
use model::order::bid::Bid;
use model::order::candidate::Candidate;
use model::repository_error::RepositoryError;
use sqlx::{QueryBuilder, query};
use std::collections::HashMap;
use tracing::instrument;
use uuid::Uuid;

use crate::Repository;
use crate::repository::{persist_asks, persist_bids};

impl<'c> MarketRepository for Repository<'c> {
    #[instrument(err(Debug), skip(self))]
    async fn get_unbound_asks(&mut self) -> Result<Vec<Ask>, Report<RepositoryError>> {
        Ok(Vec::new())
    }

    #[instrument(err(Debug), skip(self))]
    async fn get_unbound_bids(&mut self) -> Result<Vec<Bid>, Report<RepositoryError>> {
        Ok(Vec::new())
    }

    #[instrument(err(Debug), skip(self, iterator))]
    async fn persist_candidates<I>(&mut self, iterator: I) -> Result<(), Report<RepositoryError>>
    where
        I: IntoIterator<Item = Candidate>,
    {
        let mut peekable = iterator.into_iter().peekable();

        if peekable.peek().is_none() {
            return Ok(());
        };

        let mut qb = QueryBuilder::new("INSERT INTO candidate ");

        qb.push_values(peekable, |mut b, m| {
            b.push_bind(*m.get_id())
                .push_bind(*m.get_ask().get_id())
                .push_bind(*m.get_bid().get_id());
        });

        qb.build()
            .execute(&mut *self.conn)
            .await
            .change_context(RepositoryError::DatabaseError)?;

        Ok(())
    }
}
