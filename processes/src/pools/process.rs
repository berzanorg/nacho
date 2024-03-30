use nacho_data_structures::Pool;
use nacho_pools_db::PoolsDb;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process(path: &str) -> Processor {
    let path = path.to_owned();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut pools_db = PoolsDb::new(path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::GetPool {
                    sender,
                    base_token_id,
                    quote_token_id,
                } => {
                    let pool = pools_db
                        .get(&base_token_id, &quote_token_id)
                        .await
                        .map(|pool| {
                            (
                                pool.base_token_amount,
                                pool.quote_token_amount,
                                pool.total_liqudity_points,
                            )
                        });

                    sender.send(pool.ok()).unwrap();
                }
                Request::GetPools { sender } => {
                    let pools = pools_db.get_many().await.map(|pools| {
                        pools
                            .into_iter()
                            .map(|pool| {
                                (
                                    pool.base_token_id,
                                    pool.quote_token_id,
                                    pool.base_token_amount,
                                    pool.quote_token_amount,
                                    pool.total_liqudity_points,
                                )
                            })
                            .collect()
                    });

                    sender.send(pools.ok()).unwrap();
                }
                Request::GetWitness {
                    sender,
                    base_token_id,
                    quote_token_id,
                } => {
                    let single_witness = pools_db
                        .get_single_witness(&base_token_id, &quote_token_id)
                        .await;

                    sender.send(single_witness.ok()).unwrap();
                }
                Request::GetNewWitness { sender } => {
                    let new_witness = pools_db.get_new_single_witness().await;

                    sender.send(new_witness.ok()).unwrap();
                }
                Request::PushPool {
                    sender,
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount,
                    total_liqudity_points,
                } => {
                    let result = pools_db
                        .push(&Pool {
                            base_token_id,
                            quote_token_id,
                            base_token_amount,
                            quote_token_amount,
                            total_liqudity_points,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdatePool {
                    sender,
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount,
                    total_liqudity_points,
                } => {
                    let result = pools_db
                        .update(&Pool {
                            base_token_id,
                            quote_token_id,
                            base_token_amount,
                            quote_token_amount,
                            total_liqudity_points,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::PushLeaf {
                    sender,
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount,
                    total_liqudity_points,
                } => {
                    let result = pools_db
                        .push_leaf(&Pool {
                            base_token_id,
                            quote_token_id,
                            base_token_amount,
                            quote_token_amount,
                            total_liqudity_points,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateLeaf {
                    sender,
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount,
                    total_liqudity_points,
                } => {
                    let result = pools_db
                        .update_leaf(&Pool {
                            base_token_id,
                            quote_token_id,
                            base_token_amount,
                            quote_token_amount,
                            total_liqudity_points,
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
