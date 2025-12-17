#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("database connection failure")]
    DatabaseError,

    #[error("unexpedted result after running query")]
    UnexpectedResult,

    #[error("root entity not found")]
    RootEntityNotFound,
}
