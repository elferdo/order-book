use anyhow::Result;
use model::order::bid::Bid;
use model::order::repository::OrderRepository;
use repositories::Repository;
use rstest::*;
use sqlx::postgres::PgPoolOptions;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ImageExt, runners::AsyncRunner},
};
use uuid::{ContextV7, Timestamp, Uuid};

#[rstest]
#[tokio::test]
async fn test1_test() -> Result<()> {
    let postgres_instance = Postgres::default()
        .with_init_sql(include_str!("markets.sql").to_string().into_bytes())
        .with_user("fernando")
        .with_password("postgres")
        .with_tag("17.7")
        .start()
        .await?;

    let connection_string = format!(
        "postgres://fernando:postgres@{}:{}/postgres",
        postgres_instance.get_host().await?,
        postgres_instance.get_host_port_ipv4(5432).await?
    );

    let pool = PgPoolOptions::new().connect(&connection_string).await?;

    let mut conn = pool.acquire().await?;

    let mut repo = Repository::new(&mut conn).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);
    let id = Uuid::new_v7(timestamp);

    let bid = Bid::new(timestamp, id, 1.23);

    repo.persist_bid(&bid).await?;

    /*
        let recover = repo.find_bid(bid.get_id()).await?;

        assert_eq!(bid.get_id(), recover.get_id());
    */
    Ok(())
}
