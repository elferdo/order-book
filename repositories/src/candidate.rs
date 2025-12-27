use error_stack::{Report, ResultExt};
use sqlx::Row;

use model::repository_error::RepositoryError;
use model::{
    lock_mode::LockMode,
    order::{
        ask::Ask,
        bid::Bid,
        candidate::{Approval, Candidate},
        candidate_repository::CandidateRepository,
    },
    user::user::User,
};
use sqlx::{QueryBuilder, query};
use tracing::instrument;

use crate::Repository;

impl<'c> CandidateRepository for Repository<'c> {
    #[instrument(skip(self, iterator))]
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

        let _ = qb.build().execute(&mut *self.conn).await;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn archive_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> Result<(), Report<RepositoryError>> {
        let q = query!(
            "INSERT INTO candidate_archive VALUES ($1, $2)",
            candidate.get_ask().get_id(),
            candidate.get_bid().get_id()
        );

        let result = q
            .execute(&mut *self.conn)
            .await
            .change_context(RepositoryError::DatabaseError)?;

        if result.rows_affected() != 1 {
            Err(Report::new(RepositoryError::UnexpectedResult))
        } else {
            Ok(())
        }
    }

    #[instrument(skip(self))]
    async fn find_candidates_by_user(
        &mut self,
        user: &User,
    ) -> Result<Vec<Candidate>, Report<RepositoryError>> {
        let result = query!("SELECT candidate.id, candidate.ask, candidate.bid, ask.price as ask_price, bid.price as bid_price,
COALESCE(approval.ask, FALSE) as approval_ask, COALESCE(approval.bid, FALSE) as approval_bid FROM candidate JOIN ask ON candidate.ask = ask.id JOIN bid ON candidate.bid = bid.id LEFT JOIN approval ON approval.candidate = candidate.id WHERE ask.user = $1 OR bid.user = $1", *user.get_id())
            .fetch_all(&mut *self.conn)
            .await;

        let candidate_rows = result.change_context(RepositoryError::UnexpectedResult)?;

        let candidates = candidate_rows
            .iter()
            .map(|row| {
                let ask = Ask::with(row.ask, *user.get_id(), row.ask_price);
                let bid = Bid::with(row.bid, *user.get_id(), row.bid_price);
                let approval = Approval {
                    ask: row.approval_ask.expect("protected by coalesce in query"),
                    bid: row.approval_bid.expect("protected by coalesce in query"),
                };

                Candidate::with(row.id, ask, bid, approval)
            })
            .collect();

        Ok(candidates)
    }

    async fn persist_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> Result<(), Report<RepositoryError>> {
        query!(
            "INSERT INTO candidate (id, ask, bid) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            candidate.get_id(),
            candidate.get_ask().get_id(),
            candidate.get_bid().get_id(),
        )
        .execute(&mut *self.conn)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

        query!(
            "INSERT INTO approval (candidate, ask, bid) VALUES ($1, $2, $3) ON CONFLICT (candidate) DO UPDATE SET candidate = EXCLUDED.candidate, ask = EXCLUDED.ask, bid = EXCLUDED.bid",
            candidate.get_id(),
            candidate.get_ask_approval(),
            candidate.get_bid_approval(),
        )
        .execute(&mut *self.conn)
        .await.change_context(
        RepositoryError::UnexpectedResult) ?;

        Ok(())
    }

    #[instrument(skip(self, lock_mode))]
    async fn find_candidate(
        &mut self,
        lock_mode: LockMode,
        id: &uuid::Uuid,
    ) -> Result<Candidate, Report<RepositoryError>> {
        let mut qb = QueryBuilder::new(
            "SELECT candidate.id, candidate.ask, candidate.bid, ask.price as ask_price, ask.user as ask_user, bid.price as bid_price, bid.user as bid_user,
COALESCE(approval.ask, FALSE) as approval_ask, COALESCE(approval.bid, FALSE) as approval_bid FROM candidate JOIN ask ON candidate.ask = ask.id JOIN bid ON candidate.bid = bid.id LEFT JOIN approval ON approval.candidate = candidate.id WHERE candidate.id = ",
        );

        qb.push_bind(*id);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShare => {
                qb.push(" FOR KEY SHARE OF candidate;");
            }
        };

        let result = qb.build().fetch_one(&mut *self.conn).await;
        let row = result.change_context(RepositoryError::UnexpectedResult)?;

        let ask = Ask::with(row.get("ask"), row.get("ask_user"), row.get("ask_price"));
        let bid = Bid::with(row.get("bid"), row.get("bid_user"), row.get("bid_price"));
        let approval = Approval {
            ask: row.get("approval_ask"),
            bid: row.get("approval_bid"),
        };

        let candidate = Candidate::with(row.get("id"), ask, bid, approval);

        Ok(candidate)
    }

    async fn remove_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> Result<(), Report<RepositoryError>> {
        query!(
            "DELETE FROM approval WHERE candidate = $1",
            *candidate.get_id()
        )
        .execute(&mut *self.conn)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

        let _result = query!("DELETE FROM candidate WHERE id = $1", *candidate.get_id())
            .execute(&mut *self.conn)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        Ok(())
    }
}
