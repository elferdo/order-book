use model::ask::Ask;
use sqlx::{Postgres, Transaction, query};
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

pub async fn get_ask<'a>(
    mut transaction: Arc<Transaction<'a, Postgres>>,
    id: &Uuid,
) -> Result<Ask, RepositoryError> {
    let ask = query!("select * from ask where id = $1", id)
        .fetch_one(&mut **Arc::get_mut(&mut transaction).unwrap())
        .await?;

    Ok(Ask::new(ask.user, ask.price))
}

pub async fn persist_ask<'a>(
    mut transaction: Arc<Transaction<'a, Postgres>>,
    ask: &Ask,
) -> Result<(), RepositoryError> {
    query!(
        "INSERT INTO ask VALUES ($1, $2, $3)",
        ask.get_id(),
        ask.get_user_id(),
        ask.get_price()
    )
    .execute(&mut **Arc::get_mut(&mut transaction).unwrap())
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
