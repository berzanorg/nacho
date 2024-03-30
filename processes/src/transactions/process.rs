use nacho_transactions_db::TransactionsDb;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process(path: &str) -> Processor {
    let path = path.to_owned();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut transactions_db = TransactionsDb::new(path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::GetTxCount { sender } => {
                    let tx_count = transactions_db.get_tx_count().await;

                    sender.send(tx_count.ok()).unwrap();
                }
                Request::GetTxStatus { sender, tx_id } => {
                    let tx_status = transactions_db.get_status(tx_id).await;

                    sender.send(tx_status.ok()).unwrap();
                }
                Request::RejectTx { sender, tx_id } => {
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
                Request::SetProvedUntil {
                    sender,
                    until_tx_id,
                } => {
                    let result = transactions_db.set_proved_until(until_tx_id).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::SetSettledUntil {
                    sender,
                    until_tx_id,
                } => {
                    let result = transactions_db.set_settled_until(until_tx_id).await;

                    sender.send(result.ok()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
