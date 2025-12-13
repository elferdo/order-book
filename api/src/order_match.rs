use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::lock_mode::LockMode;
use model::repository::UserRepository;
use model::repository::{OrderRepository, OrderRepositoryError};
use model::{order_match::Match, repository::OrderMatchRepository};
use repositories::Repository;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::instrument;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Debug, Deserialize)]
pub struct AskRequest {
    pub price: f32,
}

#[derive(Serialize)]
struct OrderMatchSummary {
    pub id: Uuid,
    pub ask: Uuid,
    pub bid: Uuid,
    pub price: f32,
}

impl From<Match> for OrderMatchSummary {
    fn from(value: Match) -> Self {
        let id = *value.get_id();
        let ask = *value.get_ask().get_id();
        let bid = *value.get_bid().get_id();
        let price = value.get_price();

        Self {
            id,
            ask,
            bid,
            price,
        }
    }
}

async fn order_match_to_json<'c>(
    order_match: &Match,
    repo: &mut Repository<'c>,
) -> Result<OrderMatchSummary, OrderRepositoryError> {
    let ask = repo.find_ask(order_match.get_ask().get_id()).await?;
    let bid = repo.find_bid(order_match.get_bid().get_id()).await?;

    let j = OrderMatchSummary {
        id: *order_match.get_id(),
        ask: *ask.get_id(),
        bid: *bid.get_id(),
        price: order_match.get_price(),
    };

    todo!()
}

#[instrument(skip(state))]
pub async fn get_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut conn = state
        .pool
        .acquire()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let mut repo = Repository::new(&mut conn).await;

    let user = repo
        .find_user(LockMode::KeyShare, &user_id)
        .await
        .map_err(|_| ApiError::UserNotFound)?;

    let order_matches: Vec<OrderMatchSummary> = repo
        .find_order_matches_by_user(&user)
        .await
        .map_err(|_| ApiError::DatabaseError)?
        .into_iter()
        .map(|m| m.into())
        .collect();

    //let result = order_matches.iter().map(|om|
    Ok(Json::from(json!(order_matches)))
}
