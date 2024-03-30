use thiserror::Error;

/// The error type that is used for queue errors.
#[derive(Error, Debug)]
pub enum DynamicQueueError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{} doesn't have a parent directory specified", .path)]
    NoParentDirectorySpecified { path: String },
}
