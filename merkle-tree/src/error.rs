use thiserror::Error;

/// The error type that is used for Merkle tree errors.
#[derive(Error, Debug)]
pub enum MerkleTreeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("this error was supposed to never happen")]
    Infallible,
    #[error("Index you are trying to access doesn't exist in the Merkle tree.")]
    IndexDoesntExist,
    #[error("Index you are trying to acces isn't accessible yet.")]
    UnusableIndex,
}
