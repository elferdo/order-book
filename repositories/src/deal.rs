use model::deal::repository::{DealRepository, DealRepositoryError};

use crate::Repository;

impl<'c> DealRepository for Repository<'c> {
    async fn persist_deal(&mut self, _deal: &model::deal::Deal) -> Result<(), DealRepositoryError> {
        todo!()
    }
}
