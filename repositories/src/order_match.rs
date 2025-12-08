use model::order_match::Match;
use sqlx::{Database, Postgres, Transaction, query};
use thiserror::Error;
use uuid::Uuid;

pub async fn get_order_match<'a>(
    transaction: &mut Transaction<'a, Postgres>,
    ask: &Uuid,
    bid: &Uuid,
) -> Result<Match, RepositoryError> {
    let order_match = query!("select * from match where ask = $1 and bid = $2", ask, bid)
        .fetch_one(&mut **transaction)
        .await?;

    Ok(Match::new(order_match.ask, order_match.bid))
}

pub async fn persist_order_match(
    conn: &mut <Postgres as Database>::Connection,
    order_match: &Match,
) -> Result<(), RepositoryError> {
    query!(
        "INSERT INTO match VALUES ($1, $2)",
        order_match.get_ask(),
        order_match.get_bid(),
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("user error")]
    UserError(#[from] super::user::RepositoryError),
}
