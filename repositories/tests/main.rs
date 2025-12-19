use anyhow::Result;
use model::user::user::User;
use model::{lock_mode::LockMode, user::repository::UserRepository};
use repositories::Repository;
use rstest::*;
use sqlx::{PgPool, Pool, postgres::PgPoolOptions, query};
use uuid::{ContextV7, Timestamp, Uuid};

#[sqlx::test]
async fn user_with_no_orders(pool: PgPool) -> Result<()> {
    let mut conn = pool.acquire().await?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let user = User::new(timestamp);

    repo.persist_user(&user).await?;

    let recover = repo
        .find_user(model::lock_mode::LockMode::KeyShare, user.get_id())
        .await?;

    assert_eq!(user.get_id(), recover.get_id());

    Ok(())
}

#[sqlx::test]
async fn user_with_ask(pool: PgPool) -> Result<()> {
    let mut conn = pool.acquire().await?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut user = User::new(timestamp);

    let _ = user.ask(timestamp, 4.32);

    repo.persist_user(&user).await?;

    let recover = repo
        .find_user(model::lock_mode::LockMode::KeyShare, user.get_id())
        .await?;

    assert_eq!(user.get_id(), recover.get_id());

    Ok(())
}

#[sqlx::test(fixtures("first_user"))]
async fn sqlx_test(pool: PgPool) -> Result<()> {
    let mut a = pool.acquire().await?;

    let mut repo = Repository::new(&mut a).await;

    let id = Uuid::parse_str("019b36f8-bb74-7ad3-8a02-465301b72d92")?;

    let user = repo.find_user(LockMode::None, &id).await?;

    assert_eq!(*user.get_id(), id);

    Ok(())
}
