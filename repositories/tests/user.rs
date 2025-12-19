use anyhow::Result;
use model::repository_error::RepositoryError;
use model::user::user::User;
use model::{lock_mode::LockMode, user::repository::UserRepository};
use repositories::Repository;
use sqlx::{PgPool, query};
use uuid::{ContextV7, Timestamp, Uuid};

#[sqlx::test]
async fn persist_user(pool: PgPool) -> Result<()> {
    let mut conn = pool.acquire().await?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let user = User::new(timestamp);

    repo.persist_user(&user).await?;

    let recover = query!("SELECT * FROM public.user WHERE id = $1", user.get_id())
        .fetch_one(&mut *conn)
        .await?;

    assert_eq!(*user.get_id(), recover.id);

    Ok(())
}

#[sqlx::test]
async fn persist_user_with_ask(pool: PgPool) -> Result<()> {
    let mut conn = pool.acquire().await?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut user = User::new(timestamp);

    let _ = user.ask(timestamp, 4.32);

    repo.persist_user(&user).await?;

    let recover = query!(
        "SELECT * FROM ask WHERE user = $1",
        user.get_id().to_string()
    )
    .fetch_one(&mut *conn)
    .await?;

    assert_eq!(4.32, recover.price);

    Ok(())
}

#[sqlx::test(fixtures("first_user"))]
async fn find_user_when_id_exists(pool: PgPool) -> Result<()> {
    let mut a = pool.acquire().await?;

    let mut repo = Repository::new(&mut a).await;

    let id = Uuid::parse_str("019b36f8-bb74-7ad3-8a02-465301b72d92")?;

    let user = repo.find_user(LockMode::None, &id).await?;

    assert_eq!(*user.get_id(), id);

    Ok(())
}

#[sqlx::test(fixtures("first_user", "asks"))]
async fn find_user_when_id_does_not_exist(pool: PgPool) -> Result<()> {
    let mut a = pool.acquire().await?;

    let mut repo = Repository::new(&mut a).await;

    let id = Uuid::parse_str("019b37bd-e9ef-742a-995a-d49255ce41f3")?;

    let user = repo.find_user(LockMode::None, &id).await;

    assert!(matches!(user, Err(RepositoryError::DatabaseError(_))));

    Ok(())
}
