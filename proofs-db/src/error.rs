use thiserror::Error;

/// The error type that is used for Proofs DB errors.
#[derive(Error, Debug)]
pub enum ProofsDbError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
