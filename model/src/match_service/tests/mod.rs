mod repository_mock;

use super::*;
use anyhow::Result;
use repository_mock::RepositoryMock;
use uuid::{ContextV7, Uuid};

#[tokio::test]
async fn given_one_ask_and_one_matching_bid_then_one_candidate() -> Result<()> {
    let mut repo = RepositoryMock::default();

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let ask = Ask::new(timestamp, Uuid::new_v7(timestamp), 1.23);

    generate_candidates_for_ask(timestamp, &mut repo, &ask).await?;

    assert_eq!(repo.persisted_candidates.len(), 1);

    let candidate = &repo.persisted_candidates[0];

    assert_eq!(*candidate.get_ask(), ask);
    assert_eq!(candidate.get_bid().get_price(), 2.34);

    Ok(())
}
