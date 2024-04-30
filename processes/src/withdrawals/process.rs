use nacho_withdrawals_db::WithdrawalsDb;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process() -> Processor {
    let withdrawals_db_path = std::env::var("NACHO_WITHRAWALS_DB_PATH").unwrap();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut withdrawals_db = WithdrawalsDb::new(withdrawals_db_path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::Set {
                    sender,
                    index,
                    withdrawal,
                } => {
                    let result = withdrawals_db.set(index, &withdrawal).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::Get { sender, index } => {
                    let result = withdrawals_db.get(index).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::GetRoot { sender } => {
                    let result = withdrawals_db.get_root().await.map(|root| root.into());

                    sender.send(result.ok()).unwrap();
                }
                Request::GetWitness { sender, index } => {
                    let result = withdrawals_db.get_witness(index).await;

                    sender.send(result.ok()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
