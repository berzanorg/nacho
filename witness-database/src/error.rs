use thiserror::Error;

pub type Result<T> = std::result::Result<T, WitnessDatabaseError>;

#[derive(Error, Debug)]
pub enum WitnessDatabaseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("this error was supposed to never happen")]
    Infallible,
    #[error("index {} doesn't exist and highest possible index is {}", .given_index, .highest_possible_index)]
    NonExistentIndex {
        given_index: u64,
        highest_possible_index: u64,
    },
    #[error("index {} must be used before index {}", .usable_index, .given_index)]
    UnusableIndex { given_index: u64, usable_index: u64 },
}
