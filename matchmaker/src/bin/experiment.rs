use error_stack::{FutureExt, Report, ResultExt};
use matchmaker::Market;
use order::{ask::Ask, bid::Bid};
use rand::{distr::Uniform, prelude::*, rng};
use rand_distr::Normal;
use uuid::{ContextV7, Timestamp, Uuid};

#[derive(Debug, thiserror::Error)]
enum ExperimentError {
    #[error("experiment error")]
    Error,
}

fn main() -> Result<(), Report<ExperimentError>> {
    let mut best_ask = 40.0f32;
    let mut best_ask_price = 00.0;

    let mut best_bid = 1000.0f32;
    let mut best_bid_price = 100.0;

    for _ in 1..300 {
        let context = ContextV7::new();

        let timestamp = Timestamp::now(context);

        let ask_price_dist =
            Uniform::new(best_ask.max(10.0), 300.0).change_context(ExperimentError::Error)?;
        let bid_price_dist =
            Uniform::new(0.0, best_bid.min(50.0)).change_context(ExperimentError::Error)?;

        let user_id = Uuid::new_v7(timestamp);

        let asks = std::iter::repeat_with(|| ask_price_dist.sample(&mut rng()))
            .take(10)
            .map(|p| Ask::new(timestamp, user_id, p))
            .collect();

        let bids = std::iter::repeat_with(|| bid_price_dist.sample(&mut rng()))
            .take(100)
            .map(|p| Bid::new(timestamp, user_id, p))
            .collect();

        let mut market = Market::new(asks, bids);

        let _candidates = market
            .run(timestamp)
            .change_context(ExperimentError::Error)?;

        (best_ask, best_ask_price) =
            _candidates
                .iter()
                .fold((best_ask, best_ask_price), |(bb, bp), c| {
                    if c.get_price() > bp {
                        (c.get_ask().get_price(), c.get_price())
                    } else {
                        (bb, bp)
                    }
                });

        (best_bid, best_bid_price) =
            _candidates
                .iter()
                .fold((best_bid, best_bid_price), |(bb, bp), c| {
                    if c.get_price() < bp {
                        (c.get_bid().get_price(), c.get_price())
                    } else {
                        (bb, bp)
                    }
                });

        println!("{best_ask} {best_ask_price} {best_bid} {best_bid_price}");

        /*
        for c in _candidates {
            println!(
                "{}, {}, {}",
                c.get_ask().get_price(),
                c.get_bid().get_price(),
                c.get_price()
            );
        }
        */
    }

    Ok(())
}
