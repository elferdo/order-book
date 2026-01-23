use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum CucumberError {
    #[error("acquire error")]
    AcquireError,

    #[error("insert user error")]
    InsertUserError,

    #[error("transaction error")]
    TransactionError,

    #[error("error in test")]
    Error,

    #[error("could not parse parameter")]
    ParameterParseError,

    #[error("seller not found")]
    SellerNotFound,

    #[error("buyer not found")]
    BuyerNotFound,
}
