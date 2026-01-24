#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BusinessError {
    #[error("user not found in the database")]
    UserNotFound,

    #[error("user could not be persisted")]
    UserPersistenceError,

    #[error("candidate not found in the database")]
    CandidateNotFound,

    #[error("error generating candidates")]
    MatchingError,

    #[error("ask not found in the database")]
    AskNotFound,

    #[error("database error")]
    DatabaseError,
}
