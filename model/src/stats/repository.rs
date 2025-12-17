use crate::repository_error::RepositoryError;

pub trait StatsRepository {
    fn buy_price(&mut self) -> impl Future<Output = Result<f32, RepositoryError>>;
    fn sell_price(&mut self) -> impl Future<Output = Result<f32, RepositoryError>>;
}
