use model::bid::Bid;
use sqlx::{Database, Postgres, query};
use thiserror::Error;
use uuid::Uuid;

pub async fn get_bid(
    conn: &mut <Postgres as Database>::Connection,
    id: &Uuid,
) -> Result<Bid, RepositoryError> {
    let bid = query!("select * from bid where id = $1", id)
        .fetch_one(&mut *conn)
        .await?;

    Ok(Bid::new(bid.user, bid.price))
}

pub async fn persist_bid(
    conn: &mut <Postgres as Database>::Connection,
    bid: &Bid,
) -> Result<(), RepositoryError> {
    query!(
        "INSERT INTO bid VALUES ($1, $2, $3)",
        bid.get_id(),
        bid.get_user_id(),
        bid.get_price()
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
