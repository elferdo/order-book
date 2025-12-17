use crate::apierror::ApiError;
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::deal::Deal;
use model::deal::repository::DealRepository;
use model::lock_mode::LockMode;
use model::user::repository::UserRepository;
use repositories::Repository;
use serde::Serialize;
use serde_json::{Value, json};
use tracing::instrument;
use uuid::Uuid;

#[derive(Serialize)]
struct DealSummary {
    pub id: Uuid,
    pub buyer: Uuid,
    pub seller: Uuid,
    pub price: f32,
}

impl From<Deal> for DealSummary {
    fn from(value: Deal) -> Self {
        let id = *value.get_id();
        let buyer = *value.get_buyer_id();
        let seller = *value.get_seller_id();
        let price = value.get_price();

        Self {
            id,
            buyer,
            seller,
            price,
        }
    }
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

    let deals: Vec<DealSummary> = repo
        .find_deals_by_user(&user)
        .await
        .map_err(|_| ApiError::DatabaseError)?
        .into_iter()
        .map(|m| m.into())
        .collect();

    Ok(Json::from(json!(deals)))
}
