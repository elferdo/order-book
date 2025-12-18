use std::collections::HashMap;

use model::repository_error::RepositoryError;
use model::{
    lock_mode::LockMode,
    user::{repository::UserRepository, user::User},
};
use sqlx::{QueryBuilder, Row, query};
use uuid::Uuid;

use crate::Repository;

impl<'c> UserRepository for Repository<'c> {
    async fn find_user(&mut self, lock_mode: LockMode, id: &Uuid) -> Result<User, RepositoryError> {
        let mut qb = QueryBuilder::new("SELECT * FROM public.user WHERE id = ");
        qb.push_bind(id);

        match lock_mode {
            LockMode::None => {}
            LockMode::KeyShare => {
                qb.push(" FOR KEY SHARE;");
            }
        };

        let user = qb.build().fetch_one(&mut *self.conn).await?;

        let asks: HashMap<_, _> = self
            .find_asks(id)
            .await?
            .into_iter()
            .map(|ask| (*ask.get_id(), ask))
            .collect();

        let bids: HashMap<_, _> = self
            .find_bids(id)
            .await?
            .into_iter()
            .map(|bid| (*bid.get_id(), bid))
            .collect();

        Ok(User::with(user.get("id"), asks, bids))
    }

    async fn persist_user(&mut self, user: &User) -> Result<(), RepositoryError> {
        query!("INSERT INTO public.user (id) VALUES ($1)", user.get_id())
            .execute(&mut *self.conn)
            .await?;

        let asks = user.asks();
        let bids = user.bids();

        self.persist_asks(asks).await?;
        self.persist_bids(bids).await?;

        Ok(())
    }

    async fn delete_user(&mut self, user: &User) -> Result<(), RepositoryError> {
        let result = query!("DELETE FROM public.user where id = $1", user.get_id())
            .execute(&mut *self.conn)
            .await?;

        if result.rows_affected() < 1 {
            Err(RepositoryError::UnexpectedResult)
        } else {
            Ok(())
        }
    }
}
