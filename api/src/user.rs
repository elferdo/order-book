use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Json,
    extract::{Path, State},
};
use model::user::User;
use model::{lock_mode::LockMode, repository::UserRepository};
use repositories::Repository;
use serde_json::{Value, json};
use tracing::{debug, instrument};
use uuid::{ContextV7, Timestamp, Uuid};

use crate::apierror::ApiError;

#[instrument(skip(state))]
pub async fn post_handler(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let mut a = state
        .pool
        .acquire()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let mut repo = Repository::new(&mut a).await;

    let context = ContextV7::new();
    let timestamp = Timestamp::now(&context);

    let user = User::new(timestamp);

    match repo.persist_user(&user).await {
        Ok(_) => Ok(Json::from(json!({"id":user.get_id()}))),
        Err(e) => {
            debug!("error");

            let result = match e {
                model::repository::UserRepositoryError::DatabaseError => ApiError::DatabaseError,
                model::repository::UserRepositoryError::UserError => ApiError::UserNotFound,
            };

            Err(result)
        }
    }
}

#[instrument(skip(state))]
#[axum::debug_handler]
pub async fn delete_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut a = state
        .pool
        .acquire()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let mut repo = Repository::new(&mut a).await;

    let user = repo
        .find_user(LockMode::None, &id)
        .await
        .map_err(|e| match e {
            model::repository::UserRepositoryError::DatabaseError => ApiError::DatabaseError,
            model::repository::UserRepositoryError::UserError => ApiError::UserNotFound,
        })?;

    match repo.delete_user(&user).await {
        Ok(_) => Ok(Json::from(json!({"id":user.get_id()}))),
        Err(e) => {
            debug!("error");

            let result = match e {
                model::repository::UserRepositoryError::DatabaseError => ApiError::DatabaseError,
                model::repository::UserRepositoryError::UserError => ApiError::UserNotFound,
            };

            Err(result)
        }
    }
}
