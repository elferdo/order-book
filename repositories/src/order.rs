use error_stack::{Report, ResultExt};
use model::lock_mode::LockMode;
use model::order::ask::Ask;
use model::order::bid::Bid;
use model::order::repository::OrderRepository;
use model::repository_error::RepositoryError;
use sqlx::QueryBuilder;
use sqlx::{Row, query};

use crate::Repository;

impl<'c> OrderRepository for Repository<'c> {
    async fn find_asks_not_above(
        &mut self,
        lock_mode: LockMode,
        bid: &Bid,
    ) -> Result<Vec<Ask>, Report<RepositoryError>> {
        /* asks that were already a candidate match for this bid */

        let mut qb = QueryBuilder::new(
            "WITH already_matched AS (
                   SELECT ask.id FROM ask
                   LEFT JOIN candidate_archive
                       ON candidate_archive.ask = ask.id
                   WHERE candidate_archive.bid = ",
        );

        qb.push_bind(bid.get_id());

        /* asks that are not already bound to another candidate */

        qb.push(
            "),
                  free_asks AS (
                  SELECT ask.* FROM ask
                  LEFT JOIN candidate
                      ON candidate.ask = ask.id
                  WHERE candidate.bid IS NULL)

                  SELECT free_asks.* FROM free_asks
                  LEFT JOIN already_matched
                      ON already_matched.id = free_asks.id
                  WHERE already_matched IS NULL
                  AND price <= ",
        );

        qb.push_bind(bid.get_price());

        /* Don't match a user's own ask */

        qb.push(" AND free_asks.user <> ");
        qb.push_bind(bid.get_user_id());
        qb.push(";");

        /*
                match lock_mode {
                    LockMode::None => {}
                    LockMode::KeyShare => {
                        qb.push(" FOR KEY SHARE OF ask;");
                    }
                };
        */
        let ask_rows = qb
            .build()
            .fetch_all(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        let asks: Vec<_> = ask_rows
            .into_iter()
            .map(|r| Ask::with(r.get("id"), r.get("user"), r.get("price")))
            .collect();

        Ok(asks)
    }

    async fn find_bids_not_below(
        &mut self,
        lock_mode: LockMode,
        ask: &Ask,
    ) -> Result<Vec<Bid>, Report<RepositoryError>> {
        /* bids that were already a candidate match for this ask */

        let mut qb = QueryBuilder::new(
            "WITH already_matched AS (
                   SELECT bid.id FROM bid
                   LEFT JOIN candidate_archive
                       ON candidate_archive.bid = bid.id
                   WHERE candidate_archive.ask = ",
        );

        qb.push_bind(ask.get_id());

        /* bids that are not already bound to another candidate */

        qb.push(
            "),
                  free_bids AS (
                  SELECT bid.* FROM bid
                  LEFT JOIN candidate
                      ON candidate.bid = bid.id
                  WHERE candidate.ask IS NULL)

                  SELECT free_bids.* FROM free_bids
                  LEFT JOIN already_matched
                      ON already_matched.id = free_bids.id
                  WHERE already_matched IS NULL
                  AND price >= ",
        );

        qb.push_bind(ask.get_price());

        /* Don't match a user's own bid */

        qb.push(" AND free_bids.user <> ");
        qb.push_bind(ask.get_user_id());
        qb.push(";");

        /*
                match lock_mode {
                    LockMode::None => {}
                    LockMode::KeyShare => {
                        qb.push(" FOR KEY SHARE OF bid;");
                    }
                };
        */
        let bid_rows = qb
            .build()
            .fetch_all(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        let bids: Vec<_> = bid_rows
            .into_iter()
            .map(|r| Bid::with(r.get("id"), r.get("user"), r.get("price")))
            .collect();

        Ok(bids)
    }

    async fn remove_ask(&mut self, ask: &Ask) -> Result<(), Report<RepositoryError>> {
        query!("DELETE FROM ask WHERE id = $1;", *ask.get_id())
            .execute(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        Ok(())
    }

    async fn remove_bid(&mut self, bid: &Bid) -> Result<(), Report<RepositoryError>> {
        query!("DELETE FROM bid WHERE id = $1;", *bid.get_id())
            .execute(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        Ok(())
    }
}
