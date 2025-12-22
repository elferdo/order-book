mod apierror;
mod ask;
mod bid;
mod candidate;
mod deal;
mod stats;
mod user;

use anyhow::Result;
use appconfig::appstate::AppState;
use axum::{
    Router,
    routing::{delete, get, post},
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = appconfig::config::read()?;
    let state = AppState::new(&config).await?;

    let app = Router::new()
        .route("/user", post(user::post_handler))
        .route("/user/{id}", delete(user::delete_handler))
        .route("/user/{id}/bid", post(bid::post_handler))
        .route("/user/{id}/ask", post(ask::post_handler))
        .route("/user/{id}/candidate", get(candidate::get_handler))
        .route("/user/{id}/deal", get(deal::get_handler))
        .route(
            "/user/{user_id}/candidate/{candidate_id}/approve",
            post(candidate::approve_post_handler),
        )
        .route(
            "/user/{user_id}/candidate/{candidate_id}/reject",
            post(candidate::reject_post_handler),
        )
        .route("/stats/buy_price", get(stats::buy_price_get_handler))
        .route("/stats/sell_price", get(stats::sell_price_get_handler))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
