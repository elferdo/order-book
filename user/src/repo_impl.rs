use error_stack::{Report, ResultExt};
use order::{ask::Ask, bid::Bid};
use sqlx::{PgConnection, QueryBuilder, query, query_as};
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::instrument;
use uuid::Uuid;

use crate::repository_error::RepositoryError;
use crate::{repository::UserRepository, user::User};

impl UserRepository for PgConnection {
    #[instrument(err, skip(self))]
    async fn find_user(&mut self, id: &Uuid) -> Result<User, Report<RepositoryError>> {
        let result = query!("SELECT * FROM public.user WHERE id = $1", id)
            .fetch_one(&mut *self)
            .await;

        let user_id = result.change_context(RepositoryError::RootEntityNotFound)?;

        let asks: HashMap<_, _> = find_asks(&mut *self, id)
            .await?
            .into_iter()
            .map(|ask| (*ask.get_id(), ask))
            .collect();

        let bids: HashMap<_, _> = find_bids(&mut *self, id)
            .await?
            .into_iter()
            .map(|bid| (*bid.get_id(), bid))
            .collect();

        Ok(User::with(user_id.id, asks, bids))
    }

    #[instrument(skip(self))]
    async fn persist_user(&mut self, user: &User) -> Result<(), Report<RepositoryError>> {
        query!(
            "INSERT INTO public.user (id) VALUES ($1) ON CONFLICT DO NOTHING",
            user.get_id()
        )
        .execute(&mut *self)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

        let asks = user.asks();
        let bids = user.bids();

        persist_asks(&mut *self, asks).await?;
        persist_bids(&mut *self, bids).await?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn delete_user(&mut self, user: &User) -> Result<(), Report<RepositoryError>> {
        let result = query!("DELETE FROM public.user where id = $1", user.get_id())
            .execute(self)
            .await
            .change_context(RepositoryError::UnexpectedResult)?;

        if result.rows_affected() < 1 {
            Err(Report::new(RepositoryError::UnexpectedResult))
        } else {
            Ok(())
        }
    }
}

pub async fn find_asks(
    conn: &mut PgConnection,
    user_id: &Uuid,
) -> Result<Vec<Ask>, Report<RepositoryError>> {
    let asks = query_as!(
        Ask,
        "SELECT * FROM ask WHERE user = $1",
        user_id.to_string()
    )
    .fetch_all(conn)
    .await
    .change_context(RepositoryError::UnexpectedResult)?;

    Ok(asks)
}

pub async fn find_bids(
    conn: &mut PgConnection,
    user_id: &Uuid,
) -> Result<Vec<Bid>, Report<RepositoryError>> {
    let bids = query_as!(
        Bid,
        "SELECT * FROM bid WHERE user = $1",
        user_id.to_string()
    )
    .fetch_all(conn)
    .await
    .change_context(RepositoryError::UnexpectedResult)?;

    Ok(bids)
}

#[instrument(skip(conn))]
pub async fn persist_asks<'a, 'c, T: Debug + Iterator<Item = &'a Ask>>(
    conn: &mut PgConnection,
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
        .execute(conn)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

    if result.rows_affected() < 1 {
        Err(Report::new(RepositoryError::UnexpectedResult))
    } else {
        Ok(())
    }
}

#[instrument(skip(conn))]
pub async fn persist_bids<'b, 'c, T: Debug + Iterator<Item = &'b Bid>>(
    conn: &mut PgConnection,
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
        .execute(conn)
        .await
        .change_context(RepositoryError::UnexpectedResult)?;

    if result.rows_affected() < 1 {
        Err(Report::new(RepositoryError::UnexpectedResult))
    } else {
        Ok(())
    }
}
