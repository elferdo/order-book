mod apierror;
mod ask;
mod bid;
mod order_match;
mod user;

use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    routing::{delete, get, post}, Router
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = appconfig::config::read()?;
    let state = AppState::new(&config).await?;

    let app = Router::new()
        .route("/user", post(user::post_handler))
        .route("/user/{id}", delete(user::delete_handler))
        .route("/user/{id}/bid", post(bid::post_handler))
        .route("/user/{id}/ask", post(ask::post_handler))
        .route("/user/{id}/match", get(order_match::get_handler))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
