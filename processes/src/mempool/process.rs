use nacho_mempool::Mempool;
use tokio::sync::mpsc;

use super::{Processor, Request};

pub fn process(path: &str) -> Processor {
    let path = path.to_owned();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        let mut mempool = Mempool::new(path).await.unwrap();

        while let Some(request) = receiver.recv().await {
            match request {
                Request::Push {
                    sender,
                    transaction,
                } => {
                    let result = mempool.push(&transaction).await;

                    sender.send(result.ok()).unwrap();
                }
                Request::Pop { sender } => {
                    let maybe_transaction = mempool.pop().await;

                    sender.send(maybe_transaction.ok().flatten()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
