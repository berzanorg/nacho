use nacho_balances_db::BalancesDb;
use nacho_data_structures::Balance;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process(path: &str) -> Processor {
    let path = path.to_owned();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut balances_db = BalancesDb::new(path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::GetTokenAmount {
                    sender,
                    owner,
                    token_id,
                } => {
                    let balance = balances_db
                        .get(&owner, &token_id)
                        .await
                        .map(|balance| balance.token_amount);

                    sender.send(balance.ok()).unwrap();
                }
                Request::GetBalances { sender, owner } => {
                    let balances = balances_db.get_many(&owner).await.map(|balances| {
                        balances
                            .into_iter()
                            .map(|balance| (balance.token_id, balance.token_amount))
                            .collect()
                    });

                    sender.send(balances.ok()).unwrap();
                }
                Request::GetSingleWitness {
                    sender,
                    owner,
                    token_id,
                } => {
                    let single_witness = balances_db.get_single_witness(&owner, &token_id).await;

                    sender.send(single_witness.ok()).unwrap();
                }
                Request::GetDoubleWitness {
                    sender,
                    owner,
                    base_token_id,
                    quote_token_id,
                } => {
                    let double_witness = balances_db
                        .get_double_witness(&owner, &base_token_id, &owner, &quote_token_id)
                        .await;

                    sender.send(double_witness.ok()).unwrap();
                }
                Request::GetNewWitness { sender } => {
                    let new_witness = balances_db.get_new_single_witness().await;

                    sender.send(new_witness.ok()).unwrap();
                }
                Request::PushBalance {
                    sender,
                    owner,
                    token_id,
                    token_amount,
                } => {
                    let result = balances_db
                        .push(&Balance {
                            owner,
                            token_id,
                            token_amount,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateBalance {
                    sender,
                    owner,
                    token_id,
                    token_amount,
                } => {
                    let result = balances_db
                        .update(&Balance {
                            owner,
                            token_id,
                            token_amount,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::PushLeaf {
                    sender,
                    owner,
                    token_id,
                    token_amount,
                } => {
                    let result = balances_db
                        .push_leaf(&Balance {
                            owner,
                            token_id,
                            token_amount,
                        })
                        .await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateLeaf {
                    sender,
                    owner,
                    token_id,
                    token_amount,
                } => {
                    let result = balances_db
                        .update_leaf(&Balance {
                            owner,
                            token_id,
                            token_amount,
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