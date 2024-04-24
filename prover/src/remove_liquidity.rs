use nacho_data_structures::{Balance, Liquidity, Pool, U256};

use crate::{ProverError, Result};

pub fn remove_liquidity(
    base_token_balance: &mut Balance,
    quote_token_balance: &mut Balance,
    liquidity: &mut Liquidity,
    pool: &mut Pool,
    points: U256,
    base_token_amount_limit: u64,
    quote_token_amount_limit: u64,
) -> Result<()> {
    let base_token_amount =
        (points.clone() * pool.base_token_amount.into()) / pool.total_liqudity_points.clone();
    let base_token_amount: u64 = (&base_token_amount)
        .try_into()
        .map_err(|_| ProverError::Overflow)?;

    let quote_token_amount =
        (points.clone() * pool.quote_token_amount.into()) / pool.total_liqudity_points.clone();
    let quote_token_amount: u64 = (&quote_token_amount)
        .try_into()
        .map_err(|_| ProverError::Overflow)?;

    if points > liquidity.points {
        return Err(ProverError::NotEnoughLiquidty);
    }

    if base_token_amount > base_token_balance.token_amount {
        return Err(ProverError::NotEnoughBalance);
    }

    if quote_token_amount > quote_token_balance.token_amount {
        return Err(ProverError::NotEnoughBalance);
    }

    if base_token_amount > base_token_amount_limit {
        return Err(ProverError::LimitExceeded);
    }

    if quote_token_amount > quote_token_amount_limit {
        return Err(ProverError::LimitExceeded);
    }

    liquidity.points = liquidity.points.clone() - points.clone();

    pool.total_liqudity_points = pool.total_liqudity_points.clone() - points;

    pool.base_token_amount -= base_token_amount;
    pool.quote_token_amount -= quote_token_amount;
    base_token_balance.token_amount += base_token_amount;
    quote_token_balance.token_amount += quote_token_amount;

    Ok(())
}
