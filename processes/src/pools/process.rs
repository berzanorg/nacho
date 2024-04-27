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
                    let pool = pools_db.get(&base_token_id, &quote_token_id).await;

                    sender.send(pool.ok()).unwrap();
                }
                Request::GetPools { sender } => {
                    let pools = pools_db.get_many().await;

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
                Request::PushPool { sender, pool } => {
                    let result = pools_db.push(&pool).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdatePool { sender, pool } => {
                    let result = pools_db.update(&pool).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::PushLeaf { sender, pool } => {
                    let result = pools_db.push_leaf(&pool).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateLeaf { sender, pool } => {
                    let result = pools_db.update_leaf(&pool).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::GetRoot { sender } => {
                    let result = pools_db.get_root().await;

                    sender.send(result.ok().map(|root| root.into())).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
