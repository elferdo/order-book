use anyhow::Result;
use appconfig::appstate::{self, AppState};
use axum::{Json, Router, extract::State, routing::post};
use model::user::User;
use serde_json::{Value, json};
use tracing::{debug, error, info, instrument};
use tracing_subscriber::EnvFilter;

#[instrument(skip(state))]
async fn users_post_handler(State(state): State<AppState>) -> Result<Json<Value>, Json<Value>> {
    debug!("");

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

#[instrument]
async fn bids_post_handler(body: Json<Value>) -> Result<Json<Value>, Json<Value>> {
    dbg!(body);

    debug!("hola");

    Ok(Json::from(json!("ok")))
    // Err(Json(err))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = appconfig::config::read()?;
    let state = AppState::new(&config).await?;

    let app = Router::new()
        .route("/user", post(users_post_handler))
        .route("/bid", post(bids_post_handler))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
