use error_stack::Report;

use crate::{deal::Deal, repository_error::RepositoryError, user::user::User};

pub trait DealRepository {
    fn persist_deal(
        &mut self,
        deal: &Deal,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>;

    fn find_deals_by_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<Vec<Deal>, Report<RepositoryError>>>;
}
