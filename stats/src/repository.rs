use error_stack::Report;

use crate::repository_error::RepositoryError;

pub trait StatsRepository {
    fn buy_price(&mut self) -> impl Future<Output = Result<f32, Report<RepositoryError>>>;
    fn sell_price(&mut self) -> impl Future<Output = Result<f32, Report<RepositoryError>>>;
}
