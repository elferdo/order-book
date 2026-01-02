use std::collections::{BTreeSet, HashSet};

use error_stack::Report;
use error_stack::ResultExt;
use model::repository_error::RepositoryError;
use model::user::repository::UserRepository;
use model::user::user::User;
use repositories::Repository;
use sqlx::{PgPool, query};
use thiserror::Error;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Error, Debug)]
#[error("error running test")]
struct TestError;

#[sqlx::test]
async fn persist_user(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let user = User::new(timestamp);

    repo.persist_user(&user).await.change_context(TestError)?;

    let recover = query!("SELECT * FROM public.user WHERE id = $1", user.get_id())
        .fetch_one(&mut *conn)
        .await
        .change_context(TestError)?;

    assert_eq!(*user.get_id(), recover.id);

    Ok(())
}

#[sqlx::test(fixtures("first_user"))]
async fn persist_user_with_ask(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let mut repo = Repository::new(&mut *conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut user = repo
        .find_user(
            &Uuid::parse_str("019b3788-2ded-7f19-8191-9018a3939f60").change_context(TestError)?,
        )
        .await
        .change_context(TestError)?;

    let _ = user.ask(timestamp, 4.32);

    repo.persist_user(&user).await.change_context(TestError)?;

    let recover = query!("SELECT * FROM ask")
        .fetch_one(&mut *conn)
        .await
        .change_context(TestError)?;

    assert_eq!(4.32, recover.price);

    Ok(())
}

#[sqlx::test(fixtures("first_user"))]
async fn persist_user_with_more_than_one_ask(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let mut repo = Repository::new(&mut *conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut user = repo
        .find_user(
            &Uuid::parse_str("019b3788-2ded-7f19-8191-9018a3939f60").change_context(TestError)?,
        )
        .await
        .change_context(TestError)?;

    let prices = vec![1.23, 4.32, 5.67];

    for price in &prices {
        let _ = user.ask(timestamp, *price);
    }

    repo.persist_user(&user).await.change_context(TestError)?;

    let recover = query!("SELECT * FROM ask")
        .fetch_all(&mut *conn)
        .await
        .change_context(TestError)?;

    let recovered_prices: Vec<_> = recover.iter().map(|r| r.price).collect();

    for price in &prices {
        assert!(recovered_prices.contains(price));
    }

    for price in &recovered_prices {
        assert!(prices.contains(price));
    }

    Ok(())
}

#[sqlx::test(fixtures("first_user"))]
async fn persist_user_with_more_than_one_bid(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let mut repo = Repository::new(&mut *conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut user = repo
        .find_user(
            &Uuid::parse_str("019b3788-2ded-7f19-8191-9018a3939f60").change_context(TestError)?,
        )
        .await
        .change_context(TestError)?;

    let prices = vec![1.23, 4.32, 5.67];

    for price in &prices {
        let _ = user.bid(timestamp, *price);
    }

    repo.persist_user(&user).await.change_context(TestError)?;

    let recover = query!("SELECT * FROM bid")
        .fetch_all(&mut *conn)
        .await
        .change_context(TestError)?;

    let recovered_prices: Vec<_> = recover.iter().map(|r| r.price).collect();

    for price in &prices {
        assert!(recovered_prices.contains(price));
    }

    for price in &recovered_prices {
        assert!(prices.contains(price));
    }

    Ok(())
}

#[sqlx::test(fixtures("first_user"))]
async fn persist_user_with_bid(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let mut repo = Repository::new(&mut *conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let mut user = repo
        .find_user(
            &Uuid::parse_str("019b3788-2ded-7f19-8191-9018a3939f60").change_context(TestError)?,
        )
        .await
        .change_context(TestError)?;

    let _ = user.bid(timestamp, 4.32);

    repo.persist_user(&user).await.change_context(TestError)?;

    let recover = query!("SELECT * FROM bid")
        .fetch_one(&mut *conn)
        .await
        .change_context(TestError)?;

    assert_eq!(4.32, recover.price);

    Ok(())
}

#[sqlx::test(fixtures("first_user"))]
async fn find_user_when_id_exists(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut a = pool.acquire().await.change_context(TestError)?;

    let mut repo = Repository::new(&mut a).await;

    let id = Uuid::parse_str("019b36f8-bb74-7ad3-8a02-465301b72d92").change_context(TestError)?;

    let user = repo.find_user(&id).await.change_context(TestError)?;

    assert_eq!(*user.get_id(), id);

    Ok(())
}

#[sqlx::test(fixtures("first_user", "asks"))]
async fn find_user_when_id_does_not_exist(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut a = pool.acquire().await.change_context(TestError)?;

    let mut repo = Repository::new(&mut a).await;

    let id = Uuid::parse_str("019b37bd-e9ef-742a-995a-d49255ce41f3").change_context(TestError)?;

    let user = repo.find_user(&id).await;

    assert!(matches!(user, Err(_)));

    Ok(())
}
