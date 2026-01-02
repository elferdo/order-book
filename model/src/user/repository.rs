use error_stack::Report;
use uuid::Uuid;

use crate::repository_error::RepositoryError;

use super::user::User;

pub trait UserRepository {
    fn find_user(
        &mut self,
        id: &Uuid,
    ) -> impl Future<Output = Result<User, Report<RepositoryError>>>;

    fn persist_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>;

    fn delete_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<(), Report<RepositoryError>>>;
}
