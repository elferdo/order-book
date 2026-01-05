mod agent;

use std::{sync::Arc, time::Duration};

use anyhow::Result;
use rand::{distr::Uniform, prelude::*, rng};
use reqwest::Url;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::agent::Agent;

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
    // tracing_subscriber::fmt().pretty().init();
    tracing_subscriber::fmt().json().init();
    // let j = json!("{}");
    let client = reqwest::Client::new();

    // let result = c.post(u).body(j.to_string()).send().await?;
    // let t = result.text().await?;
    // let user: User = serde_json::from_str(&t)?;
    // let user_id = Arc::new(user.id);

    /*
        let u = Url::parse("http://localhost:5000/user/019b079b-941f-7343-b80a-21acb0cda274/ask")?;

        let data = json!({"price": 1200.0});

        let result = client.post(u.clone()).json(&data).send().await?;
        let result = client.post(u).json(&data).send().await?;


        let c = client.clone();

        let handle = tokio::spawn(async move {
            let u = Url::parse("http://localhost:5000/user/019b07a2-6cd5-7ad0-93db-53c19101ecf3/bid")
                .unwrap();

            let data = json!({"price": 1200.0});

            let result = c.post(u).json(&data).send().await.unwrap();
        });

        handles.push(handle);

        let c = client.clone();

        let handle = tokio::spawn(async move {
            let u = Url::parse("http://localhost:5000/user/019b07a2-76b2-76a0-a45c-b22f5fa485ab/bid")
                .unwrap();

            let data = json!({"price": 1200.0});

            let result = c.post(u).json(&data).send().await.unwrap();
        });

        handles.push(handle);
    */

    let mut handles = Vec::new();

    let should_run = Arc::new(true);

    for _ in 1..100 {
        let c = should_run.clone();

        let handle = tokio::spawn(async {
            let mut agent = Agent::new(c).await.unwrap();

            let _ = agent.run().await;
        });

        handles.push(handle);
    }

    for handle in handles {
        let result = tokio::join!(handle);

        result.0?;
    }
    Ok(())
}
