use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{Json, extract::State};
use model::user::User;
use serde_json::{Value, json};
use tracing::{debug, instrument};

#[instrument(skip(state))]
pub async fn users_post_handler(State(state): State<AppState>) -> Result<Json<Value>, Json<Value>> {
    let user = User::new();

    let urepo = repositories::user::Repository::new(state.pool);

    match urepo.persist_user(&user).await {
        Ok(_) => Ok(Json::from(json!({"id": user.get_id()}))),
        Err(_) => {
            debug!("error");
            Err(Json::from(json!("error")))
        }
    }
}
