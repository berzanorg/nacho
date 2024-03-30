use nacho_data_structures::Liquidity;
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
                Request::GetLiquidityPoints {
                    sender,
                    provider,
                    base_token_id,
                    quote_token_id,
                } => {
                    let liquidity_points = liquidities_db
                        .get(&provider, &base_token_id, &quote_token_id)
                        .await
                        .map(|liquidity| liquidity.points);

                    sender.send(liquidity_points.ok()).unwrap();
                }
                Request::GetLiquidities { sender, provider } => {
                    let liquidities = liquidities_db.get_many(&provider).await.map(|liquidities| {
                        liquidities
                            .into_iter()
                            .map(|liquidities| {
                                (
                                    liquidities.base_token_id,
                                    liquidities.quote_token_id,
                                    liquidities.points,
                                )
                            })
                            .collect()
                    });

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
                Request::PushLiquidity {
                    sender,
                    provider,
                    base_token_id,
                    quote_token_id,
                    points,
                } => {
                    let result = liquidities_db
                        .push(&Liquidity {
                            provider,
                            base_token_id,
                            quote_token_id,
                            points,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateLiquidity {
                    sender,
                    provider,
                    base_token_id,
                    quote_token_id,
                    points,
                } => {
                    let result = liquidities_db
                        .update(&Liquidity {
                            provider,
                            base_token_id,
                            quote_token_id,
                            points,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::PushLeaf {
                    sender,
                    provider,
                    base_token_id,
                    quote_token_id,
                    points,
                } => {
                    let result = liquidities_db
                        .push_leaf(&Liquidity {
                            provider,
                            base_token_id,
                            quote_token_id,
                            points,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateLeaf {
                    sender,
                    provider,
                    base_token_id,
                    quote_token_id,
                    points,
                } => {
                    let result = liquidities_db
                        .update_leaf(&Liquidity {
                            provider,
                            base_token_id,
                            quote_token_id,
                            points,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
