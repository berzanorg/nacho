use nacho_transactions_db::TransactionsDb;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process() -> Processor {
    let transaction_db_path = std::env::var("NACHO_TRANSACTIONS_DB_PATH").unwrap();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut transactions_db = TransactionsDb::new(transaction_db_path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::GetTotalTxCount { sender } => {
                    let tx_count = transactions_db.get_tx_count().await;

                    sender.send(tx_count.ok()).unwrap();
                }

                Request::GetTxStatus { sender, tx_id } => {
                    let tx_status = transactions_db.get_status(tx_id).await;

                    sender.send(tx_status.ok()).unwrap();
                }

                Request::AddNewTx { sender } => {
                    let tx_status = transactions_db.add_new_tx().await;

                    sender.send(tx_status.ok()).unwrap();
                }

                Request::SetRejected { sender, tx_id } => {
                    let result = transactions_db.set_rejected(tx_id).await;

                    sender.send(result.ok()).unwrap();
                }

                Request::SetExecutedUntil {
                    sender,
                    until_tx_id,
                } => {
                    let result = transactions_db.set_executed_until(until_tx_id).await;

                    sender.send(result.ok()).unwrap();
                }

                Request::GetExecutedUntil { sender } => {
                    let result = transactions_db.get_executed_until().await;

                    sender.send(result.ok()).unwrap();
                }

                Request::SetProvedUntil {
                    sender,
                    until_tx_id,
                } => {
                    let result = transactions_db.set_proved_until(until_tx_id).await;

                    sender.send(result.ok()).unwrap();
                }

                Request::GetProvedUntil { sender } => {
                    let result = transactions_db.get_proved_until().await;

                    sender.send(result.ok()).unwrap();
                }

                Request::SetSettledUntil {
                    sender,
                    until_tx_id,
                } => {
                    let result = transactions_db.set_settled_until(until_tx_id).await;

                    sender.send(result.ok()).unwrap();
                }

                Request::GetSettledUntil { sender } => {
                    let result = transactions_db.get_settled_until().await;

                    sender.send(result.ok()).unwrap();
                }

                Request::SetMergedUntil {
                    sender,
                    until_tx_id,
                } => {
                    let result = transactions_db.set_merged_until(until_tx_id).await;

                    sender.send(result.ok()).unwrap();
                }

                Request::GetMergedUntil { sender } => {
                    let result = transactions_db.get_merged_until().await;

                    sender.send(result.ok()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
