use std::{sync::Arc, time::Duration};

use anyhow::Result;
use rand::{distr::Uniform, prelude::*, rng};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct User {
    id: Uuid,
}

struct BidDistribution {
    pub price: Uniform<f32>,
}

#[derive(Serialize)]
struct Bid {
    pub price: f32,
}

impl Distribution<Bid> for BidDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Bid {
        let price = self.price.sample(rng);

        Bid { price }
    }
}

async fn post_bid(user_id: &Uuid) -> Result<()> {
    let price = Uniform::new(10.0, 100.0)?.sample(&mut rng());

    let u = Url::parse(&format!("http://127.0.0.1:5000/user/{user_id}/bid").to_string())?;

    let bidj = json!({"user": user_id, "price": price});

    let c = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(30))
        .build()?;
    let result = c.post(u).json(&bidj).send().await?;

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
            let result = post_bid(&u).await;
        });

        handles.push(handle);
    }

    for handle in handles {
        tokio::join!(handle);
    }
    Ok(())
}
