use thiserror::Error;

/// The error type that is used for Merkle tree errors.
#[derive(Error, Debug)]
pub enum MerkleTreeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("this error was supposed to never happen")]
    Infallible,
    #[error("highest possible index is {}", .0)]
    NonExistentIndex(u64),
    #[error("usable index is {}", .0)]
    UnusableIndex(u64),
    #[error("mistaken order for witness x3`")]
    MistakenOrderX3,
    #[error("mistaken order for witness x4`")]
    MistakenOrderX4,
}
