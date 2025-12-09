use model::{
    lock_mode::LockMode,
    repository::{UserRepository, UserRepositoryError},
    user::User,
};
use sqlx::{QueryBuilder, Row, query};
use uuid::Uuid;

use crate::Repository;

impl<'c> UserRepository for Repository<'c> {
    async fn find_user(
        &mut self,
        lock_mode: LockMode,
        id: &Uuid,
    ) -> Result<User, UserRepositoryError> {
        let mut qb = QueryBuilder::new("SELECT * FROM public.user WHERE id = ");
        qb.push_bind(id);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShared => {
                qb.push(" FOR KEY SHARE;");
            }
        };

        let user = qb
            .build()
            .fetch_one(&mut *self.conn)
            .await
            .map_err(|_| UserRepositoryError::UserError)?;

        Ok(User::new_as(user.get("id")))
    }

    async fn persist_user(&mut self, user: &User) -> Result<(), UserRepositoryError> {
        query!("INSERT INTO public.user (id) VALUES ($1)", user.get_id())
            .execute(&mut *self.conn)
            .await
            .map_err(|_| UserRepositoryError::DatabaseError)?;

        Ok(())
    }

    async fn delete_user(&mut self, user: &User) -> Result<(), UserRepositoryError> {
        let result = query!("DELETE FROM public.user where id = $1", user.get_id())
            .execute(&mut *self.conn)
            .await
            .map_err(|_| UserRepositoryError::DatabaseError)?;

        if result.rows_affected() < 1 {
            Err(UserRepositoryError::DatabaseError)
        } else {
            Ok(())
        }
    }
}
