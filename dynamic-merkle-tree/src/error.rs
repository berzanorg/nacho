use thiserror::Error;

/// The error type that represents the errors that can occur while working with `DynamicMerkleTree`.
#[derive(Error, Debug)]
pub enum DynamicMerkleTreeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("this error was supposed to never happen")]
    Infallible,
    #[error("Index you are trying to access doesn't exist in the dynamic Merkle tree.")]
    IndexDoesntExist,
    #[error("Index you are trying to acces isn't accessible yet.")]
    UnusableIndex,
}
