use thiserror::Error;

pub trait StatsRepository {
    fn buy_price(&mut self) -> impl Future<Output = Result<f32, StatsRepositoryError>>;
    fn sell_price(&mut self) -> impl Future<Output = Result<f32, StatsRepositoryError>>;
}

#[derive(Debug, Error)]
pub enum StatsRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}
