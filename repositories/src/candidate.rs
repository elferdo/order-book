use sqlx::Row;
use std::collections::HashMap;

use model::{
    lock_mode::LockMode,
    order::{
        ask::Ask,
        bid::Bid,
        candidate::Candidate,
        candidate_repository::{CandidateRepository, CandidateRepositoryError},
    },
    user::user::User,
};
use sqlx::{QueryBuilder, query};
use tracing::{debug, instrument};

use crate::Repository;

impl<'c> CandidateRepository for Repository<'c> {
    #[instrument(skip(self, iterator))]
    async fn persist_candidates<I>(&mut self, iterator: I) -> Result<(), CandidateRepositoryError>
    where
        I: IntoIterator<Item = Candidate>,
    {
        debug!("inserting candidate");

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

        let _ = qb
            .build()
            .execute(&mut *self.conn)
            .await
            .map_err(|_| CandidateRepositoryError::DatabaseError);

        Ok(())
    }

    async fn find_candidates_by_user(
        &mut self,
        user: &User,
    ) -> Result<Vec<Candidate>, CandidateRepositoryError> {
        let candidate_rows = query!("SELECT candidate.id, candidate.ask, candidate.bid, ask.price as ask_price, bid.price as bid_price FROM candidate JOIN ask ON candidate.ask = ask.id JOIN bid ON candidate.bid = bid.id WHERE ask.user = $1 OR bid.user = $1", user.get_id())
            .fetch_all(&mut *self.conn)
            .await
            .map_err(|_| CandidateRepositoryError::DatabaseError)?;

        let candidates = candidate_rows
            .iter()
            .map(|r| {
                let ask = Ask::with(r.ask, *user.get_id(), r.ask_price);
                let bid = Bid::with(r.bid, *user.get_id(), r.bid_price);
                Candidate::with(r.id, ask, bid)
            })
            .collect();

        Ok(candidates)
    }

    async fn persist_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> Result<(), CandidateRepositoryError> {
        query!(
            "INSERT INTO candidate (id, ask, bid) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            candidate.get_id(),
            candidate.get_ask().get_id(),
            candidate.get_bid().get_id(),
        )
        .execute(&mut *self.conn)
        .await
        .map_err(|_| CandidateRepositoryError::DatabaseError)?;

        query!(
            "INSERT INTO approval (candidate, ask, bid) VALUES ($1, $2, $3) ON CONFLICT (candidate) DO UPDATE SET candidate = EXCLUDED.candidate, ask = EXCLUDED.ask, bid = EXCLUDED.bid",
            candidate.get_id(),
            candidate.get_ask_approval(),
            candidate.get_bid_approval(),
        )
        .execute(&mut *self.conn)
        .await
        .map_err(|_| CandidateRepositoryError::DatabaseError)?;

        Ok(())
    }

    async fn find_candidate(
        &mut self,
        lock_mode: LockMode,
        id: &uuid::Uuid,
    ) -> Result<Candidate, CandidateRepositoryError> {
        let mut qb = QueryBuilder::new(
            "SELECT candidate.id, candidate.ask, candidate.bid, ask.price as ask_price, ask.user as ask_user, bid.price as bid_price, bid.user as bid_user FROM candidate JOIN ask ON candidate.ask = ask.id JOIN bid ON candidate.bid = bid.id WHERE candidate.id = ",
        );

        qb.push_bind(*id);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShare => {
                qb.push(" FOR KEY SHARE;");
            }
        };

        let row = qb
            .build()
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| CandidateRepositoryError::DatabaseError)?;

        let ask = Ask::with(row.get("ask"), row.get("ask_user"), row.get("ask_price"));
        let bid = Bid::with(row.get("bid"), row.get("bid_user"), row.get("bid_price"));

        let candidate = Candidate::with(row.get("id"), ask, bid);

        Ok(candidate)
    }
}
