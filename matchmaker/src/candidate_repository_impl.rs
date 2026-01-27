use error_stack::{IntoReport, Report, ResultExt};
use order::{ask::Ask, bid::Bid};
use sqlx::{PgConnection, QueryBuilder, query};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    candidate::{Approval, Candidate},
    candidate_repository::CandidateRepository,
    repository_error::RepositoryError,
};

impl CandidateRepository for PgConnection {
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
            .execute(self)
            .await
            .change_context(RepositoryError::DatabaseError)?;

        Ok(())
    }

    #[instrument(err(Debug), skip(self))]
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
            .execute(self)
            .await
            .change_context(RepositoryError::DatabaseError)?;

        if result.rows_affected() != 1 {
            Err(RepositoryError::UnexpectedResult.into_report())
        } else {
            Ok(())
        }
    }

    #[instrument(err(Debug), skip(self))]
    async fn find_candidates_by_user(
        &mut self,
        user_id: &Uuid,
    ) -> Result<Vec<Candidate>, Report<RepositoryError>> {
        let result = query!("SELECT candidate.id, candidate.ask, candidate.bid, ask.not_below as ask_price, bid.not_above as bid_price,
COALESCE(approval.ask, FALSE) as approval_ask, COALESCE(approval.bid, FALSE) as approval_bid FROM candidate JOIN ask ON candidate.ask = ask.id JOIN bid ON candidate.bid = bid.id LEFT JOIN approval ON approval.candidate = candidate.id WHERE ask.seller= $1 OR bid.buyer= $1",
*user_id)
            .fetch_all(self)
            .await;

        let candidate_rows = result.change_context(RepositoryError::UnexpectedResult)?;

        let candidates = candidate_rows
            .iter()
            .map(|row| {
                let ask = Ask::with(row.ask, *user_id, row.ask_price);
                let bid = Bid::with(row.bid, *user_id, row.bid_price);
                let approval = Approval {
                    ask: row.approval_ask.unwrap_or(false),
                    bid: row.approval_bid.unwrap_or(false),
                };

                Candidate::with(row.id, ask, bid, approval)
            })
            .collect();

        Ok(candidates)
    }

    #[instrument(err(Debug), skip(self))]
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
        .execute(&mut *self)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

        query!(
            "INSERT INTO approval (candidate, ask, bid) VALUES ($1, $2, $3) ON CONFLICT (candidate) DO UPDATE SET candidate = EXCLUDED.candidate, ask = EXCLUDED.ask, bid = EXCLUDED.bid",
            candidate.get_id(),
            candidate.get_ask_approval(),
            candidate.get_bid_approval(),
        )
        .execute(&mut *self)
        .await.change_context(
        RepositoryError::UnexpectedResult) ?;

        Ok(())
    }

    #[instrument(err(Debug), skip(self,))]
    async fn find_candidate(
        &mut self,
        id: &uuid::Uuid,
    ) -> Result<Candidate, Report<RepositoryError>> {
        let result = query!(
            "SELECT candidate.id, candidate.ask, candidate.bid, ask.not_below as ask_price,
ask.seller as ask_user,
bid.not_above as bid_price, bid.buyer as bid_user,
COALESCE(approval.ask, FALSE) as approval_ask, COALESCE(approval.bid, FALSE) as approval_bid FROM candidate JOIN ask ON candidate.ask = ask.id JOIN bid ON candidate.bid = bid.id LEFT JOIN approval ON approval.candidate = candidate.id WHERE candidate.id = $1",
*id).fetch_one(self).await;

        let row = result.change_context(RepositoryError::UnexpectedResult)?;

        let ask = Ask::with(row.ask, row.ask_user, row.ask_price);
        let bid = Bid::with(row.bid, row.bid_user, row.bid_price);
        let approval = Approval {
            ask: row.approval_ask.unwrap_or(false),
            bid: row.approval_bid.unwrap_or(false),
        };

        let candidate = Candidate::with(row.id, ask, bid, approval);

        Ok(candidate)
    }

    #[instrument(err(Debug), skip(self))]
    async fn remove_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> Result<(), Report<RepositoryError>> {
        query!(
            "DELETE FROM approval WHERE candidate = $1",
            *candidate.get_id()
        )
        .execute(&mut *self)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

        let _result = query!("DELETE FROM candidate WHERE id = $1", *candidate.get_id())
            .execute(&mut *self)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        Ok(())
    }
}
