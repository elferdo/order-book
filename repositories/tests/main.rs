use testcontainers_modules::{
    postgres::{self, Postgres},
    testcontainers::{ContainerRequest, ImageExt, runners::SyncRunner},
};

#[test]
fn test1() {
    let postgres_instance = Postgres::default()
        .with_init_sql(include_str!("markets.sql").to_string().into_bytes())
        .with_user("fernando")
        .with_tag("17.7")
        .start()
        .unwrap();

    let connection_string = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        postgres_instance.get_host().unwrap(),
        postgres_instance.get_host_port_ipv4(5432).unwrap()
    );
}
