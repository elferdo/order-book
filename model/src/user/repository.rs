use thiserror::Error;
use uuid::Uuid;

use crate::lock_mode::LockMode;

use super::user::User;

pub trait UserRepository {
    fn find_user(
        &mut self,
        lock_mode: LockMode,
        id: &Uuid,
    ) -> impl Future<Output = Result<User, UserRepositoryError>>;

    fn persist_user(
        &mut self,
        user: &User,
    ) -> impl Future<Output = Result<(), UserRepositoryError>>;

    fn delete_user(&mut self, user: &User)
    -> impl Future<Output = Result<(), UserRepositoryError>>;
}

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("repository error")]
    DatabaseError,

    #[error("user error")]
    UserError,
}
