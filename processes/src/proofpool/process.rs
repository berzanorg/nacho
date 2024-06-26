use nacho_proofpool::Proofpool;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process() -> Processor {
    let proofpool_path = std::env::var("NACHO_PROOFPOOL_PATH").unwrap();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut proofpool = Proofpool::new(proofpool_path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::Push {
                    sender,
                    stateful_tx: transaction,
                } => {
                    let result = proofpool.push(&transaction).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::Pop { sender } => {
                    let maybe_transaction = proofpool.pop().await;

                    sender.send(maybe_transaction.ok().flatten()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
