use std::{sync::Arc, time::Duration};

use error_stack::{Report, ResultExt};
use rand::{
    Rng, SeedableRng,
    distr::{Distribution, Uniform},
    rngs::StdRng,
};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tracing::{info, instrument};
use uuid::Uuid;

pub struct Agent {
    id: Uuid,
    rng: StdRng,
    should_run: Arc<bool>,
}

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("network error")]
    NetworkError,

    #[error("agent error")]
    Error,
}

#[derive(Deserialize)]
struct UserResponse {
    id: Uuid,
}

enum AgentAction {
    Buy,
    Sell,
    Approve,
    Reject,
}

struct AgentActionDist;

impl Distribution<AgentAction> for AgentActionDist {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AgentAction {
        let d = Uniform::new(0, 4).expect("shouldn't fail");

        match rng.sample(d) {
            0 => AgentAction::Buy,
            1 => AgentAction::Sell,
            2 => AgentAction::Approve,
            3 => AgentAction::Reject,
            _ => panic!(),
        }
    }
}

async fn post_new_user() -> Result<Uuid, Report<AgentError>> {
    let c = reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(30))
        .build()
        .change_context(AgentError::NetworkError)?;

    let u = Url::parse(&format!("http://127.0.0.1:5000/user").to_string())
        .change_context(AgentError::NetworkError)?;

    let response: UserResponse = c
        .post(u)
        .send()
        .await
        .change_context(AgentError::NetworkError)?
        .json()
        .await
        .change_context(AgentError::NetworkError)?;

    Ok(response.id)
}

impl Agent {
    pub async fn new(should_run: Arc<bool>) -> Result<Self, Report<AgentError>> {
        let rng = StdRng::from_os_rng();
        let id = post_new_user().await?;

        Ok(Self {
            id,
            rng,
            should_run,
        })
    }

    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<(), Report<AgentError>> {
        while *self.should_run {
            tokio::time::sleep(Duration::from_secs(3)).await;

            match self.rng.sample(AgentActionDist {}) {
                AgentAction::Buy => self.buy().await?,
                AgentAction::Sell => self.sell().await?,
                AgentAction::Approve => self.approve().await?,
                AgentAction::Reject => self.reject().await?,
            }
        }

        Ok(())
    }

    #[instrument(skip(self))]
    async fn buy(&mut self) -> Result<(), Report<AgentError>> {
        let c = build_client().await?;

        let price_dist = Uniform::new(0.0, 100.0).change_context(AgentError::Error)?;

        let price = self.rng.sample(price_dist);
        let bid = json!({"price": price});

        let u = Url::parse(&format!("http://127.0.0.1:5000/user/{}/bid", self.id).to_string())
            .change_context(AgentError::NetworkError)?;

        let response = c
            .post(u)
            .json(&bid)
            .send()
            .await
            .change_context(AgentError::NetworkError)?;

        info!(user = self.id.to_string(), price = price);

        Ok(())
    }

    #[instrument(skip(self))]
    async fn sell(&mut self) -> Result<(), Report<AgentError>> {
        let c = build_client().await?;

        let price_dist = Uniform::new(0.0, 100.0).change_context(AgentError::Error)?;

        let price = self.rng.sample(price_dist);
        let ask = json!({"price": price});

        let u = Url::parse(&format!("http://127.0.0.1:5000/user/{}/ask", self.id).to_string())
            .change_context(AgentError::NetworkError)?;

        let response = c
            .post(u)
            .json(&ask)
            .send()
            .await
            .change_context(AgentError::NetworkError)?;

        info!(user = self.id.to_string(), price = price);

        Ok(())
    }

    #[instrument(skip(self))]
    async fn approve(&mut self) -> Result<(), Report<AgentError>> {
        info!("approve");

        let c = build_client().await?;

        let u =
            Url::parse(&format!("http://127.0.0.1:5000/user/{}/candidate", self.id).to_string())
                .change_context(AgentError::NetworkError)?;

        let response: Vec<Candidate> = c
            .post(u)
            .send()
            .await
            .change_context(AgentError::NetworkError)?
            .json()
            .await
            .change_context(AgentError::NetworkError)?;

        info!(user = self.id.to_string(), "approve");

        Ok(())
    }

    #[instrument(skip(self))]
    async fn reject(&mut self) -> Result<(), Report<AgentError>> {
        let c = build_client().await?;

        let u =
            Url::parse(&format!("http://127.0.0.1:5000/user/{}/candidate", self.id).to_string())
                .change_context(AgentError::NetworkError)?;

        let response: Vec<Candidate> = c
            .post(u)
            .send()
            .await
            .change_context(AgentError::NetworkError)?
            .json()
            .await
            .change_context(AgentError::NetworkError)?;

        Ok(())
    }
}

async fn build_client() -> Result<Client, Report<AgentError>> {
    reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(30))
        .timeout(Duration::from_secs(30))
        .build()
        .change_context(AgentError::NetworkError)
}

#[derive(Deserialize, Serialize)]
struct Candidate {
    buyer: Uuid,
    seller: Uuid,
    price: f32,
}
