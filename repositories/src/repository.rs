use sqlx::PgConnection;

pub struct Repository<'c> {
    pub(crate) conn: &'c mut PgConnection,
}

impl<'c> Repository<'c> {
    pub async fn new(conn: &'c mut PgConnection) -> Self {
        Self { conn }
    }
}
