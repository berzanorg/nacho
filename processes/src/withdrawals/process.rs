use nacho_data_structures::FieldConversion;
use nacho_withdrawals_db::WithdrawalsDb;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process(path: &str) -> Processor {
    let path = path.to_owned();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut withdrawals_db = WithdrawalsDb::new(path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::GetWitness { sender, burn_id } => {
                    let single_witness = withdrawals_db.get_witness(burn_id).await;

                    sender.send(single_witness.ok()).unwrap();
                }
                Request::SetLeaf {
                    sender,
                    burn_id,
                    value,
                } => {
                    let result = withdrawals_db.set(burn_id, value.to_fields()[0]).await;

                    sender.send(result.ok()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
