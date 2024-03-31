use nacho_data_structures::{Balance, Burn};

use crate::{ExecutorError, Result};

pub fn burn_tokens(
    balance: &mut Balance,
    maybe_burn: Option<&mut Burn>,
    amount_to_burn: u64,
) -> Result<Option<Burn>> {
    if amount_to_burn > balance.token_amount {
        return Err(ExecutorError::NotEnoughBalance);
    }

    balance.token_amount -= amount_to_burn;

    match maybe_burn {
        Some(burn) => {
            burn.token_amount += amount_to_burn;
            Ok(None)
        }
        None => Ok(Some(Burn {
            burner: balance.owner.clone(),
            token_id: balance.token_id.clone(),
            token_amount: amount_to_burn,
        })),
    }
}
