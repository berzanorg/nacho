use nacho_balances_db::BalancesDb;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process() -> Processor {
    let balances_db_path = std::env::var("NACHO_BALANCES_DB_PATH").unwrap();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut balances_db = BalancesDb::new(balances_db_path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::GetBalance {
                    sender,
                    owner,
                    token_id,
                } => {
                    let balance = balances_db.get(&owner, &token_id).await;

                    sender.send(balance.ok()).unwrap();
                }
                Request::GetBalances { sender, owner } => {
                    let balances = balances_db.get_many(&owner).await;

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
                Request::PushBalance { sender, balance } => {
                    let result = balances_db.push(&balance).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateBalance { sender, balance } => {
                    let result = balances_db.update(&balance).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::PushLeaf { sender, balance } => {
                    let result = balances_db.push_leaf(&balance).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::UpdateLeaf { sender, balance } => {
                    let result = balances_db.update_leaf(&balance).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::GetRoot { sender } => {
                    let result = balances_db.get_root().await;

                    sender.send(result.ok().map(|root| root.into())).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
