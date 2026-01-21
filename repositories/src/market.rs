use error_stack::{Report, ResultExt};
use model::market::repository::MarketRepository;
use model::order::ask::Ask;
use model::order::bid::Bid;
use model::repository_error::RepositoryError;
use sqlx::query;
use std::collections::HashMap;
use tracing::instrument;
use uuid::Uuid;

use crate::Repository;
use crate::repository::{persist_asks, persist_bids};

impl<'c> MarketRepository for Repository<'c> {
    #[instrument(err(Debug), skip(self))]
    async fn get_unbound_asks(&mut self) -> Result<Vec<Ask>, Report<RepositoryError>> {
        Ok(Vec::new())
    }

    #[instrument(err(Debug), skip(self))]
    async fn get_unbound_bids(&mut self) -> Result<Vec<Bid>, Report<RepositoryError>> {
        Ok(Vec::new())
    }
}
