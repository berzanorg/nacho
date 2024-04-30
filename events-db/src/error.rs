use thiserror::Error;

/// The error type for Events DB operations.
#[derive(Error, Debug)]
pub enum EventsDbError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
