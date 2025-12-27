use std::fmt::Display;

use business::candidate::approve_candidate;
use error_stack::{Report, ResultExt};
use sqlx::{PgPool, query};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
#[error("test error")]
struct TestError;

#[sqlx::test(fixtures("one_candidate"))]
async fn test1(pool: PgPool) -> Result<(), Report<TestError>> {
    let mut conn = pool.acquire().await.change_context(TestError)?;

    approve_candidate(
        pool.clone(),
        Uuid::parse_str("019b5f63-7b50-7188-8062-5f678bc9a409").change_context(TestError)?,
        Uuid::parse_str("019b5f61-181d-7afe-83c6-168d9bb5e69b").change_context(TestError)?,
    )
    .await
    .change_context(TestError)?;

    approve_candidate(
        pool.clone(),
        Uuid::parse_str("019b5f5f-2ad7-7c02-ab5e-f13d608ff85a").change_context(TestError)?,
        Uuid::parse_str("019b5f61-181d-7afe-83c6-168d9bb5e69b").change_context(TestError)?,
    )
    .await
    .change_context(TestError)?;

    let result = query!("SELECT * FROM deal")
        .fetch_all(&mut *conn)
        .await
        .change_context(TestError)?;

    for i in result {
        println!("{}", i.id);
    }

    // dbg!(result);

    todo!()
}
