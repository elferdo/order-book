#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    /* It's unfortunate that we have to introduce this dependency in the domain
     * model, but it's the most convenient way to surface database errors,
     * useful for debugging.
     */
    #[error("database error")]
    DatabaseError,

    #[error("unexpedted result after running query")]
    UnexpectedResult,

    #[error("root entity not found")]
    RootEntityNotFound,
}
