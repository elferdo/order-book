use anyhow::Result;
use reqwest::Url;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct User {
    id: Uuid,
}

#[tokio::main]
async fn main() -> Result<()> {
    let j = json!("{}");
    let c = reqwest::Client::new();
    let u = Url::parse("http://localhost:5000/user")?;

    let result = c.post(u).body(j.to_string()).send().await?;

    let t = result.text().await?;

    let user: User = serde_json::from_str(&t)?;

    let bidj = json!({"user": user.id, "price": 2.34});

    println!("{bidj}");

    let user_id = user.id;
    let u = Url::parse(&format!("http://localhost:5000/user/{user_id}/ask").to_string())?;

    let result = c.post(u).json(&bidj).send().await?;

    println!("{}", result.text().await?);

    Ok(())
}
