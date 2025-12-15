use std::collections::HashMap;

use model::{
    order::candidate::Candidate,
    order::candidate_repository::{CandidateRepository, CandidateRepositoryError},
    order::{ask::Ask, bid::Bid},
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

        let asks: HashMap<_, _> = candidate_rows
            .iter()
            .map(|r| (r.ask, Ask::with(r.ask, *user.get_id(), r.ask_price)))
            .collect();

        let bids: HashMap<_, _> = candidate_rows
            .iter()
            .map(|r| (r.bid, Bid::with(r.bid, *user.get_id(), r.bid_price)))
            .collect();

        let candidates = candidate_rows
            .iter()
            .map(|r| {
                let ask = asks.get(&r.ask).unwrap();
                let bid = bids.get(&r.bid).unwrap();
                Candidate::with(r.id, *ask, *bid)
            })
            .collect();

        Ok(candidates)
    }

    async fn persist_candidate(
        &mut self,
        candidate: &Candidate,
    ) -> Result<(), CandidateRepositoryError> {
        query!(
            "INSERT INTO candidate VALUES ($1, $2, $3)",
            candidate.get_id(),
            candidate.get_ask().get_id(),
            candidate.get_bid().get_id(),
        )
        .execute(&mut *self.conn)
        .await
        .map_err(|_| CandidateRepositoryError::DatabaseError)?;

        Ok(())
    }
}
