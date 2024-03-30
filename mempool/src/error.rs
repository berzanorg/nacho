use nacho_dynamic_queue::DynamicQueueError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MempoolError {
    #[error(transparent)]
    Dd(#[from] DynamicQueueError),
}
