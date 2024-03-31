use nacho_data_structures::{Balance, Liquidity, Pool, U256};

use crate::{ExecutorError, Result};

pub fn provide_liquidity(
    base_token_balance: &mut Balance,
    quote_token_balance: &mut Balance,
    maybe_liquidity: Option<&mut Liquidity>,
    pool: &mut Pool,
    base_token_amount: u64,
    quote_token_amount_limit: u64,
) -> Result<Option<Liquidity>> {
    let liquidity_points_to_create = (pool.total_liqudity_points.clone()
        * base_token_amount.into())
        / pool.base_token_amount.into();

    let quote_token_amount: U256 = (<u64 as Into<U256>>::into(base_token_amount)
        * pool.quote_token_amount.into())
        / pool.base_token_amount.into();

    let quote_token_amount: u64 = (&quote_token_amount)
        .try_into()
        .map_err(|_| ExecutorError::Overflow)?;

    if base_token_amount > base_token_balance.token_amount {
        return Err(ExecutorError::NotEnoughBalance);
    }

    if quote_token_amount > quote_token_balance.token_amount {
        return Err(ExecutorError::NotEnoughBalance);
    }

    if quote_token_amount > quote_token_amount_limit {
        return Err(ExecutorError::LimitExceeded);
    }

    base_token_balance.token_amount -= base_token_amount;
    quote_token_balance.token_amount -= quote_token_amount;
    pool.base_token_amount += base_token_amount;
    pool.quote_token_amount += quote_token_amount;
    pool.total_liqudity_points =
        pool.total_liqudity_points.clone() + liquidity_points_to_create.clone();

    match maybe_liquidity {
        Some(liquidity) => {
            liquidity.points = liquidity.points.clone() + liquidity_points_to_create;
            Ok(None)
        }
        None => Ok(Some(Liquidity {
            provider: base_token_balance.owner.clone(),
            base_token_id: base_token_balance.token_id.clone(),
            quote_token_id: quote_token_balance.token_id.clone(),
            points: liquidity_points_to_create,
        })),
    }
}
