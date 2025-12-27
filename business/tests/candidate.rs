use business::{
    ask::new_ask,
    bid::new_bid,
    candidate::{approve_candidate, get_candidates},
};
use error_stack::{Report, ResultExt};
use sqlx::{PgPool, Row, query};
use std::fmt::Display;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
#[error("test error")]
struct TestError;

#[sqlx::test(fixtures("one_candidate"))]
async fn test1(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let seller =
        Uuid::parse_str("019b5f63-7b50-7188-8062-5f678bc9a409").change_context(TestError)?;

    let buyer =
        Uuid::parse_str("019b5f5f-2ad7-7c02-ab5e-f13d608ff85a").change_context(TestError)?;

    approve_candidate(
        pool.clone(),
        seller,
        Uuid::parse_str("019b5f61-181d-7afe-83c6-168d9bb5e69b").change_context(TestError)?,
    )
    .await
    .change_context(TestError)?;

    approve_candidate(
        pool.clone(),
        buyer,
        Uuid::parse_str("019b5f61-181d-7afe-83c6-168d9bb5e69b").change_context(TestError)?,
    )
    .await
    .change_context(TestError)?;

    let result = query!("SELECT * FROM deal")
        .fetch_all(&mut *conn)
        .await
        .change_context(TestError)?;

    assert_eq!(result.len(), 1);

    let deal = &result[0];

    assert_eq!(deal.buyer, buyer);
    assert_eq!(deal.seller, seller);

    Ok(())
}

#[sqlx::test(fixtures("one_archived_candidate"))]
async fn test2(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let seller =
        Uuid::parse_str("019b5f63-7b50-7188-8062-5f678bc9a409").change_context(TestError)?;

    let buyer =
        Uuid::parse_str("019b5f5f-2ad7-7c02-ab5e-f13d608ff85a").change_context(TestError)?;

    let ask = new_ask(pool.clone(), seller, 1.50)
        .await
        .change_context(TestError)?;

    let candidates = get_candidates(pool.clone(), seller);

    let result = query!("SELECT * FROM candidate")
        .fetch_all(&mut *conn)
        .await
        .change_context(TestError)?;

    assert_eq!(result.len(), 1);

    Ok(())
}

#[sqlx::test(fixtures("one_archived_candidate"))]
async fn test3(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let seller =
        Uuid::parse_str("019b5f63-7b50-7188-8062-5f678bc9a409").change_context(TestError)?;

    let buyer =
        Uuid::parse_str("019b5f5f-2ad7-7c02-ab5e-f13d608ff85a").change_context(TestError)?;

    let bid = new_bid(pool.clone(), buyer, 1.50)
        .await
        .change_context(TestError)?;

    let candidates = get_candidates(pool.clone(), seller);

    let result = query!("SELECT * FROM candidate")
        .fetch_all(&mut *conn)
        .await
        .change_context(TestError)?;

    assert_eq!(result.len(), 1);

    Ok(())
}

#[sqlx::test(fixtures("one_archived_candidate"))]
async fn test4(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let seller =
        Uuid::parse_str("019b5f63-7b50-7188-8062-5f678bc9a409").change_context(TestError)?;

    let buyer =
        Uuid::parse_str("019b5f5f-2ad7-7c02-ab5e-f13d608ff85a").change_context(TestError)?;

    let ask = new_ask(pool.clone(), seller, 4.0)
        .await
        .change_context(TestError)?;

    let candidates = get_candidates(pool.clone(), seller);

    let result = query!("SELECT * FROM candidate")
        .fetch_all(&mut *conn)
        .await
        .change_context(TestError)?;

    assert!(result.is_empty());

    Ok(())
}

#[sqlx::test(fixtures("one_archived_candidate"))]
async fn test5(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    let seller =
        Uuid::parse_str("019b5f63-7b50-7188-8062-5f678bc9a409").change_context(TestError)?;

    let buyer =
        Uuid::parse_str("019b5f5f-2ad7-7c02-ab5e-f13d608ff85a").change_context(TestError)?;

    let bid = new_bid(pool.clone(), buyer, 1.0)
        .await
        .change_context(TestError)?;

    let candidates = get_candidates(pool.clone(), seller);

    let result = query!("SELECT * FROM candidate")
        .fetch_all(&mut *conn)
        .await
        .change_context(TestError)?;

    assert!(result.is_empty());

    Ok(())
}
