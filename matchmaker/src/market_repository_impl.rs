use error_stack::{Report, ResultExt};
use matchmaker::repository::MarketRepository;
use order::order::ask::Ask;
use order::order::bid::Bid;
use order::order::candidate::Candidate;
use order::repository_error::RepositoryError;
use sqlx::{QueryBuilder, query_as};
use tracing::instrument;

use crate::Repository;

impl<'c> MarketRepository for Repository<'c> {
    #[instrument(err(Debug), skip(self))]
    async fn get_unbound_asks(&mut self) -> Result<Vec<Ask>, Report<RepositoryError>> {
        let asks = query_as!(Ask, "SELECT * FROM ask;")
            .fetch_all(&mut *self.conn)
            .await
            .change_context(RepositoryError::DatabaseError)?;

        Ok(asks)
    }

    #[instrument(err(Debug), skip(self))]
    async fn get_unbound_bids(&mut self) -> Result<Vec<Bid>, Report<RepositoryError>> {
        let bids = query_as!(Bid, "SELECT * FROM bid;")
            .fetch_all(&mut *self.conn)
            .await
            .change_context(RepositoryError::DatabaseError)?;

        Ok(bids)
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
