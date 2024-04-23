use thiserror::Error;

/// The error type that represents the errors that can occur while working with `StaticMerkleTree`.
#[derive(Error, Debug)]
pub enum StaticMerkleTreeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Parent directory isn't specified.")]
    ParentDirectoryNotSpecified,
    #[error("Leaf indexes are exceeded.")]
    LeafIndexesExceeded,
}
