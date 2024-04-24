use nacho_data_structures::{Balance, Pool, U256};

use crate::{ProverError, Result};

pub fn buy_tokens(
    base_token_id: U256,
    maybe_base_token_balance: Option<&mut Balance>,
    quote_token_balance: &mut Balance,
    pool: &mut Pool,
    base_token_amount: u64,
    quote_token_amount_limit: u64,
) -> Result<Option<Balance>> {
    if base_token_amount >= pool.base_token_amount {
        return Err(ProverError::NotEnoughInPool);
    }

    let k: U256 = U256::from(pool.base_token_amount) * pool.quote_token_amount.into();

    let new_pool_base_token_amount = pool.base_token_amount - base_token_amount;

    let new_pool_quote_token_amount = k / new_pool_base_token_amount.into();
    let new_pool_quote_token_amount: u64 = (&new_pool_quote_token_amount)
        .try_into()
        .map_err(|_| ProverError::Overflow)?;

    let quote_token_amount = new_pool_quote_token_amount - pool.quote_token_amount;

    let quote_token_amount_plus_fee =
        (U256::from(quote_token_amount) * 1001u64.into()) / 1000u64.into();
    let quote_token_amount_plus_fee: u64 = (&quote_token_amount_plus_fee)
        .try_into()
        .map_err(|_| ProverError::Overflow)?;

    if quote_token_amount_plus_fee > quote_token_balance.token_amount {
        return Err(ProverError::NotEnoughBalance);
    }

    if quote_token_amount_plus_fee > quote_token_amount_limit {
        return Err(ProverError::LimitExceeded);
    }

    quote_token_balance.token_amount -= quote_token_amount_plus_fee;
    pool.base_token_amount -= base_token_amount;
    pool.quote_token_amount += quote_token_amount_plus_fee;

    match maybe_base_token_balance {
        Some(base_token_balance) => {
            base_token_balance.token_amount += base_token_amount;

            Ok(None)
        }
        None => Ok(Some(Balance {
            owner: quote_token_balance.owner.clone(),
            token_id: base_token_id,
            token_amount: base_token_amount,
        })),
    }
}
