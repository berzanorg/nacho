use nacho_liquidities_db::LiquiditiesDb;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process(path: &str) -> Processor {
    let path = path.to_owned();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut liquidities_db = LiquiditiesDb::new(path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::GetLiquidity {
                    sender,
                    provider,
                    base_token_id,
                    quote_token_id,
                } => {
                    let liquidity = liquidities_db
                        .get(&provider, &base_token_id, &quote_token_id)
                        .await;

                    sender.send(liquidity.ok()).unwrap();
                }
                Request::GetLiquidities { sender, provider } => {
                    let liquidities = liquidities_db.get_many(&provider).await;

                    sender.send(liquidities.ok()).unwrap();
                }
                Request::GetWitness {
                    sender,
                    provider,
                    base_token_id,
                    quote_token_id,
                } => {
                    let single_witness = liquidities_db
                        .get_single_witness(&provider, &base_token_id, &quote_token_id)
                        .await;

                    sender.send(single_witness.ok()).unwrap();
                }
                Request::GetNewWitness { sender } => {
                    let new_witness = liquidities_db.get_new_single_witness().await;

                    sender.send(new_witness.ok()).unwrap();
                }
                Request::PushLiquidity { sender, liquidity } => {
                    let result = liquidities_db.push(&liquidity).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateLiquidity { sender, liquidity } => {
                    let result = liquidities_db.update(&liquidity).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::PushLeaf { sender, liquidity } => {
                    let result = liquidities_db.push_leaf(&liquidity).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateLeaf { sender, liquidity } => {
                    let result = liquidities_db.update_leaf(&liquidity).await;

                    sender.send(result.ok()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
