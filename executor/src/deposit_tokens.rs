use nacho_data_structures::{Address, Balance, U256};

use crate::Result;

pub fn deposit_tokens(
    maybe_balance: Option<&mut Balance>,
    amount_to_deposit: u64,
    token_id: U256,
    user_address: Address,
) -> Result<Option<Balance>> {
    match maybe_balance {
        Some(balance) => {
            balance.token_amount += amount_to_deposit;
            Ok(None)
        }
        None => Ok(Some(Balance {
            owner: user_address,
            token_id: token_id,
            token_amount: amount_to_deposit,
        })),
    }
}
