use std::path::Path;

use nacho_data_structures::Transaction;
use nacho_dynamic_queue::DynamicQueue;

use crate::ProofpoolError;

pub struct Proofpool {
    queue: DynamicQueue<264, Transaction>,
}

type Result<T> = std::result::Result<T, ProofpoolError>;

impl Proofpool {
    pub async fn new(path: impl AsRef<Path>) -> Result<Proofpool> {
        let path = path.as_ref();

        let queue = DynamicQueue::new(path).await?;

        Ok(Self { queue })
    }

    pub async fn push(&mut self, transaction: &Transaction) -> Result<()> {
        self.queue.push(transaction).await?;

        Ok(())
    }

    pub async fn pop(&mut self) -> Result<Option<Transaction>> {
        let maybe_transaction = self.queue.pop().await?;

        Ok(maybe_transaction)
    }
}
