use std::path::Path;

use nacho_data_structures::ProverMethod;
use nacho_dynamic_queue::DynamicQueue;

use crate::ProofpoolError;

pub struct Proofpool {
    queue: DynamicQueue<3291, ProverMethod>,
}

type Result<T> = std::result::Result<T, ProofpoolError>;

impl Proofpool {
    pub async fn new(path: impl AsRef<Path>) -> Result<Proofpool> {
        let path = path.as_ref();

        let queue = DynamicQueue::new(path).await?;

        Ok(Self { queue })
    }

    pub async fn push(&mut self, method: &ProverMethod) -> Result<()> {
        self.queue.push(method).await?;

        Ok(())
    }

    pub async fn pop(&mut self) -> Result<Option<ProverMethod>> {
        let maybe_method = self.queue.pop().await?;

        Ok(maybe_method)
    }
}
