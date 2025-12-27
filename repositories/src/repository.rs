use error_stack::{Report, ResultExt};
use model::{
    order::{ask::Ask, bid::Bid},
    repository_error::RepositoryError,
};
use sqlx::{PgConnection, QueryBuilder, query};
use std::fmt::Debug;
use tracing::instrument;
use uuid::Uuid;

pub struct Repository<'c> {
    pub(crate) conn: &'c mut PgConnection,
}

impl<'c> Repository<'c> {
    pub async fn new(conn: &'c mut PgConnection) -> Self {
        Self { conn }
    }

    #[instrument(skip(self))]
    pub async fn persist_asks<'a, T: Debug + Iterator<Item = &'a Ask>>(
        &mut self,
        asks: T,
    ) -> Result<(), Report<RepositoryError>> {
        let mut peekable = asks.peekable();

        if peekable.peek().is_none() {
            return Ok(());
        }

        let mut qb = QueryBuilder::new("INSERT INTO ask ");

        qb.push_values(peekable, |mut b, ask| {
            b.push_bind(*ask.get_id())
                .push_bind(*ask.get_user_id())
                .push_bind(ask.get_price());
        });

        qb.push(" ON CONFLICT DO NOTHING;");

        let query = qb.build();
        let result = query
            .execute(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        if result.rows_affected() < 1 {
            Err(Report::new(RepositoryError::UnexpectedResult))
        } else {
            Ok(())
        }
    }

    #[instrument(skip(self))]
    pub async fn persist_bids<'b, T: Debug + Iterator<Item = &'b Bid>>(
        &mut self,
        bids: T,
    ) -> Result<(), Report<RepositoryError>> {
        let mut peekable = bids.peekable();

        if peekable.peek().is_none() {
            return Ok(());
        }

        let mut qb = QueryBuilder::new("INSERT INTO bid ");

        qb.push_values(peekable, |mut b, bid| {
            b.push_bind(*bid.get_id())
                .push_bind(*bid.get_user_id())
                .push_bind(bid.get_price());
        });

        qb.push(" ON CONFLICT DO NOTHING;");

        let query = qb.build();

        let result = query
            .execute(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        if result.rows_affected() < 1 {
            Err(Report::new(RepositoryError::UnexpectedResult))
        } else {
            Ok(())
        }
    }

    pub async fn find_asks(&mut self, user_id: &Uuid) -> Result<Vec<Ask>, Report<RepositoryError>> {
        let ask_rows = query!("SELECT * FROM ask WHERE user = $1", user_id.to_string())
            .fetch_all(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        let asks = ask_rows
            .iter()
            .map(|row| Ask::with(row.id, row.user, row.price))
            .collect();

        Ok(asks)
    }

    pub async fn find_bids(&mut self, user_id: &Uuid) -> Result<Vec<Bid>, Report<RepositoryError>> {
        let bid_rows = query!("SELECT * FROM bid WHERE user = $1", user_id.to_string())
            .fetch_all(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        let bids = bid_rows
            .iter()
            .map(|row| Bid::with(row.id, row.user, row.price))
            .collect();

        Ok(bids)
    }
}
