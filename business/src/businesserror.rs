#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BusinessError {
    #[error("user not found in the database")]
    UserNotFound,

    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
}
