use uuid::Uuid;

use crate::{lock_mode::LockMode, repository_error::RepositoryError};

use super::user::User;

pub trait UserRepository {
    fn find_user(
        &mut self,
        lock_mode: LockMode,
        id: &Uuid,
    ) -> impl Future<Output = Result<User, RepositoryError>>;

    fn persist_user(&mut self, user: &User) -> impl Future<Output = Result<(), RepositoryError>>;

    fn delete_user(&mut self, user: &User) -> impl Future<Output = Result<(), RepositoryError>>;
}
