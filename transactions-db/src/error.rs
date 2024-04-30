use thiserror::Error;

/// The error type that is used for Merkle tree errors.
#[derive(Error, Debug)]
pub enum TransactionsDbError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Transaction doesn't exist.")]
    TxDoesntExist,
}
