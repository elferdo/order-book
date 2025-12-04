use anyhow::Result;
use axum::{Json, Router, routing::post};
use model::user::User;
use serde_json::{Value, json};

async fn tasks_post_handler() -> Result<Json<Value>, Json<Value>> {
    let user = User::new();

    Ok(Json::from(json!({"id": user.get_id()})))
    // Err(Json(err))
}

async fn bids_post_handler(body: Json<Value>) -> Result<Json<Value>, Json<Value>> {
    dbg!(body);

    Ok(Json::from(json!("ok")))
    // Err(Json(err))
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/user", post(tasks_post_handler))
        .route("/bid", post(bids_post_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
