use std::{sync::Arc, time::Duration};

use anyhow::Result;
use rand::{distr::Uniform, prelude::*, rng};
use reqwest::Url;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct User {
    id: Uuid,
}

async fn post_orders(user_id: &Uuid) -> Result<()> {
    let c = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(30))
        .build()?;
    let price = Uniform::new(40.0, 70.0)?.sample(&mut rng());

    let u = Url::parse(&format!("http://127.0.0.1:5000/user/{user_id}/bid").to_string())?;

    let bidj = json!({"user": user_id, "price": price});

    c.post(u).json(&bidj).send().await?;

    let price = Uniform::new(60.0, 100.0)?.sample(&mut rng());

    let u = Url::parse(&format!("http://127.0.0.1:5000/user/{user_id}/ask").to_string())?;

    let askj = json!({"user": user_id, "price": price});

    c.post(u).json(&askj).send().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let j = json!("{}");
    let c = reqwest::Client::new();
    let u = Url::parse("http://localhost:5000/user")?;

    let result = c.post(u).body(j.to_string()).send().await?;

    let t = result.text().await?;

    let user: User = serde_json::from_str(&t)?;

    let user_id = Arc::new(user.id);

    let mut handles = Vec::new();

    for _ in 1..10000 {
        let u = user_id.clone();
        let handle = tokio::spawn(async move {
            match post_orders(&u).await {
                Ok(_) => {}
                Err(_) => eprintln!("error"),
            };
        });

        handles.push(handle);
    }

    for handle in handles {
        let result = tokio::join!(handle);

        result.0?;
    }
    Ok(())
}
