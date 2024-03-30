use nacho_dynamic_queue::DynamicQueueError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProofpoolError {
    #[error(transparent)]
    Dd(#[from] DynamicQueueError),
}
