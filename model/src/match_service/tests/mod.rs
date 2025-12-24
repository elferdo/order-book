mod repository_mock;

use super::*;
use anyhow::Result;
use repository_mock::RepositoryMock;
use uuid::{ContextV7, Uuid};

#[tokio::test]
async fn given_one_ask_and_no_matching_bid_then_no_candidate() -> Result<()> {
    let mut repo = RepositoryMock::default();

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let ask = Ask::new(timestamp, Uuid::new_v7(timestamp), 1.23);

    repo.asks.push(ask);

    generate_candidates_for_ask(timestamp, &mut repo, &ask).await?;

    assert_eq!(repo.candidates.len(), 0);

    Ok(())
}

#[tokio::test]
async fn given_one_ask_and_one_matching_bid_then_one_candidate() -> Result<()> {
    let mut repo = RepositoryMock::default();

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let ask = Ask::new(timestamp, Uuid::new_v7(timestamp), 1.23);
    let bid = Bid::new(timestamp, Uuid::new_v7(timestamp), 2.34);

    repo.asks.push(ask);
    repo.bids.push(bid);

    generate_candidates_for_ask(timestamp, &mut repo, &ask).await?;

    assert_eq!(repo.candidates.len(), 1);

    let candidate = &repo.candidates[0];

    assert_eq!(*candidate.get_ask(), ask);
    assert_eq!(candidate.get_bid().get_price(), 2.34);

    Ok(())
}

#[tokio::test]
async fn given_one_ask_and_three_matching_bids_then_one_candidate() -> Result<()> {
    let mut repo = RepositoryMock::default();

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let ask = Ask::new(timestamp, Uuid::new_v7(timestamp), 1.23);
    let bid1 = Bid::new(timestamp, Uuid::new_v7(timestamp), 2.34);
    let bid2 = Bid::new(timestamp, Uuid::new_v7(timestamp), 3.45);
    let bid3 = Bid::new(timestamp, Uuid::new_v7(timestamp), 4.56);

    repo.asks.push(ask);
    repo.bids.extend([bid1, bid2, bid3]);

    generate_candidates_for_ask(timestamp, &mut repo, &ask).await?;

    assert_eq!(repo.candidates.len(), 1);

    let candidate = &repo.candidates[0];

    assert_eq!(*candidate.get_ask(), ask);
    assert_eq!(candidate.get_bid().get_price(), 2.34);

    Ok(())
}

#[tokio::test]
async fn given_three_asks_and_one_matching_bid_then_one_candidate() -> Result<()> {
    let mut repo = RepositoryMock::default();

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let ask1 = Ask::new(timestamp, Uuid::new_v7(timestamp), 2.34);
    let ask2 = Ask::new(timestamp, Uuid::new_v7(timestamp), 3.45);
    let ask3 = Ask::new(timestamp, Uuid::new_v7(timestamp), 4.56);
    let bid = Bid::new(timestamp, Uuid::new_v7(timestamp), 3.00);

    repo.asks.extend([ask1, ask2, ask3]);
    repo.bids.push(bid);

    generate_candidates_for_bid(timestamp, &mut repo, &bid).await?;

    assert_eq!(repo.candidates.len(), 1);

    let candidate = &repo.candidates[0];

    assert_eq!(*candidate.get_bid(), bid);
    assert_eq!(candidate.get_ask().get_price(), 2.34);

    Ok(())
}

#[tokio::test]
async fn given_three_asks_and_three_bids_only_one_matching_then_three_candidates() -> Result<()> {
    let mut repo = RepositoryMock::default();

    let context = ContextV7::new();
    let timestamp = Timestamp::now(context);

    let ask1 = Ask::new(timestamp, Uuid::new_v7(timestamp), 2.34);
    let ask2 = Ask::new(timestamp, Uuid::new_v7(timestamp), 3.45);
    let ask3 = Ask::new(timestamp, Uuid::new_v7(timestamp), 4.56);
    let bid1 = Bid::new(timestamp, Uuid::new_v7(timestamp), 1.00);
    let bid2 = Bid::new(timestamp, Uuid::new_v7(timestamp), 2.00);
    let bid3 = Bid::new(timestamp, Uuid::new_v7(timestamp), 7.00);

    repo.asks.extend([ask1, ask2, ask3]);

    generate_candidates_for_bid(timestamp, &mut repo, &bid1).await?;
    generate_candidates_for_bid(timestamp, &mut repo, &bid2).await?;
    generate_candidates_for_bid(timestamp, &mut repo, &bid3).await?;

    assert_eq!(repo.candidates.len(), 1);

    dbg!(&repo.candidates);

    let candidate = &repo.candidates[0];

    assert_eq!(*candidate.get_bid(), bid3);
    assert_eq!(*candidate.get_ask(), ask1);

    Ok(())
}
