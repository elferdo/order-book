use anyhow::Result;
use business::candidate::approve_candidate;
use sqlx::{PgPool, query};
use uuid::Uuid;

#[sqlx::test(fixtures("one_candidate"))]
async fn test1(pool: PgPool) -> Result<()> {
    let mut conn = pool.acquire().await?;

    approve_candidate(
        pool.clone(),
        Uuid::parse_str("019b5f63-7b50-7188-8062-5f678bc9a409")?,
        Uuid::parse_str("019b5f61-181d-7afe-83c6-168d9bb5e69b")?,
    )
    .await
    .unwrap();

    approve_candidate(
        pool.clone(),
        Uuid::parse_str("019b5f5f-2ad7-7c02-ab5e-f13d608ff85a")?,
        Uuid::parse_str("019b5f61-181d-7afe-83c6-168d9bb5e69b")?,
    )
    .await
    .unwrap();

    let result = query!("SELECT * FROM deal").fetch_all(&mut *conn).await?;

    for i in result {
        println!("{}", i.id);
    }

    // dbg!(result);

    todo!()
}
