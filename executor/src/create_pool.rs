use nacho_data_structures::{Balance, Liquidity, Pool};

use crate::{ExecutorError, Result};

pub fn create_pool(
    base_token_balance: &mut Balance,
    quote_token_balance: &mut Balance,
    base_token_amount: u64,
    quote_token_amount: u64,
) -> Result<(Pool, Liquidity)> {
    if base_token_amount > base_token_balance.token_amount {
        return Err(ExecutorError::NotEnoughBalance);
    }

    if quote_token_amount > quote_token_balance.token_amount {
        return Err(ExecutorError::NotEnoughBalance);
    }

    base_token_balance.token_amount -= base_token_amount;
    quote_token_balance.token_amount -= quote_token_amount;

    let pool = Pool {
        base_token_id: base_token_balance.token_id.clone(),
        quote_token_id: quote_token_balance.token_id.clone(),
        base_token_amount,
        quote_token_amount,
        total_liqudity_points: (base_token_amount as u128 * quote_token_amount as u128).into(),
    };

    let liquidity = Liquidity {
        base_token_id: base_token_balance.token_id.clone(),
        quote_token_id: quote_token_balance.token_id.clone(),
        provider: base_token_balance.owner.clone(),
        points: pool.total_liqudity_points.clone(),
    };

    Ok((pool, liquidity))
}
