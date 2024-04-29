use thiserror::Error;

/// The error type for static list opeartions.
#[derive(Error, Debug)]
pub enum StaticListError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Index out of bounds in the list.")]
    IndexOutOfBounds,
    #[error("Parent directory of the list isn't specified.")]
    ParentDirectoryNotSpecified,
}
