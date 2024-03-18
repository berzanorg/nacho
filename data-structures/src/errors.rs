use thiserror::Error;

/// The error type that represents the errors that can happen during arithmetic operations on `Uint64`.
#[derive(Error, Debug)]
pub enum Uint64Error {
    #[error("Uint64 is out of range")]
    OutOfRange,
}
