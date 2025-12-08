use model::bid::Bid;
use sqlx::{Postgres, Transaction, query};
use thiserror::Error;
use uuid::Uuid;

pub async fn get_bid<'a>(
    transaction: &mut Transaction<'a, Postgres>,
    id: &Uuid,
) -> Result<Bid, RepositoryError> {
    let bid = query!("select * from bid where id = $1", id)
        .fetch_one(&mut **transaction)
        .await?;

    Ok(Bid::new(bid.user, bid.price))
}

pub async fn persist_bid<'a>(
    transaction: &mut Transaction<'a, Postgres>,
    bid: &Bid,
) -> Result<(), RepositoryError> {
    query!(
        "INSERT INTO bid VALUES ($1, $2, $3)",
        bid.get_id(),
        bid.get_user_id(),
        bid.get_price()
    )
    .execute(&mut **transaction)
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
