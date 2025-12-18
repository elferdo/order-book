use anyhow::Result;
use rstest::*;
use sqlx::postgres::PgPoolOptions;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ImageExt, runners::AsyncRunner},
};
use url::Url;

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

    Ok(())
}
