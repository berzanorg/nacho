use thiserror::Error;

/// The error type that is used for Merkle tree errors.
#[derive(Error, Debug)]
pub enum DynamicListError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Index out of bounds in the list.")]
    IndexOutOfBounds,
    #[error("Parent directory of the list isn't specified.")]
    ParentDirectoryNotSpecified,
}
