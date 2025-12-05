mod apierror;
mod bid;
mod user;

use crate::{bid::bids_post_handler, user::users_post_handler};
use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{Router, routing::post};
use tracing_subscriber::EnvFilter;

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
