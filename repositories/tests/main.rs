use anyhow::Result;
use model::user::repository::UserRepository;
use model::user::user::User;
use repositories::Repository;
use rstest::*;
use sqlx::{Pool, postgres::PgPoolOptions};
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};
use uuid::{ContextV7, Timestamp, Uuid};

#[fixture]
async fn postgres_instance() -> Result<ContainerAsync<Postgres>> {
    let postgres_instance = Postgres::default()
        .with_init_sql(include_str!("markets.sql").to_string().into_bytes())
        .with_user("fernando")
        .with_password("postgres")
        .with_tag("17.7")
        .start()
        .await?;

    Ok(postgres_instance)
}

#[rstest]
#[tokio::test]
async fn user_with_no_orders(
    #[future] postgres_instance: Result<ContainerAsync<Postgres>>,
) -> Result<()> {
    let db_instance = postgres_instance.await?;

    let connection_string = format!(
        "postgres://fernando:postgres@{}:{}/postgres",
        db_instance.get_host().await?,
        db_instance.get_host_port_ipv4(5432).await?
    );

    let pool = PgPoolOptions::new().connect(&connection_string).await?;

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

#[rstest]
#[tokio::test]
async fn user_with_ask(
    #[future] postgres_instance: Result<ContainerAsync<Postgres>>,
) -> Result<()> {
    let db_instance = postgres_instance.await?;

    let connection_string = format!(
        "postgres://fernando:postgres@{}:{}/postgres",
        db_instance.get_host().await?,
        db_instance.get_host_port_ipv4(5432).await?
    );

    let pool = PgPoolOptions::new().connect(&connection_string).await?;

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
