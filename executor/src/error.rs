use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecutorError {
    #[error("Not enough balance.")]
    NotEnoughBalance,
    #[error("Not enough liquidty.")]
    NotEnoughLiquidty,
    #[error("Not enough tokens in pool.")]
    NotEnoughInPool,
    #[error("Overflow.")]
    Overflow,
    #[error("Limit exceeded.")]
    LimitExceeded,
}

pub(crate) type Result<T> = std::result::Result<T, ExecutorError>;
