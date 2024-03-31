use super::{Processor, Request};
use crate::{authenticator, balances, burns, liquidities, mempool, pools, proofpool};
use nacho_data_structures::Transaction;
use tokio::sync::mpsc;

pub fn process(
    authenticator: authenticator::Processor,
    mempool: mempool::Processor,
    proofpool: proofpool::Processor,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
) -> Processor {
    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    tokio::spawn(async move {
        while let Some(request) = receiver.recv().await {
            match request {
                Request::ExecuteNext { sender } => {
                    let result = execute_tx(
                        authenticator,
                        mempool,
                        proofpool,
                        balances,
                        pools,
                        liquidities,
                        burns,
                    )
                    .await;

                    sender.send(result.ok()).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}

pub async fn execute_tx(
    authenticator: authenticator::Processor,
    mempool: mempool::Processor,
    proofpool: proofpool::Processor,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
) -> Result<(), ()> {
    let tx = mempool.pop().await.ok_or(())?;

    let is_valid = authenticator.check_signature(tx.clone()).await.ok_or(())?;

    if !is_valid {
        return Err(());
    }

    match tx.clone() {
        Transaction::BurnTokens {
            address,
            signature: _,
            token_id,
            token_amount,
        } => {
            let mut balance = balances
                .get_balance(address.clone(), token_id.clone())
                .await
                .ok_or(())?;

            let mut maybe_burn = burns
                .get_burn_token_amount(address.clone(), token_id.clone())
                .await;

            let amount_to_burn = token_amount;

            let result =
                nacho_executor::burn_tokens(&mut balance, maybe_burn.as_mut(), amount_to_burn)
                    .map_err(|_| ())?;

            balances.update_balance(balance).await.ok_or(())?;

            match (result, maybe_burn) {
                (Some(burn), None) => {
                    burns.push_burn(burn).await.ok_or(())?;
                }
                (None, Some(burn)) => {
                    burns.update_burn(burn).await.ok_or(())?;
                }
                _ => return Err(()),
            }
        }
        Transaction::CreatePool {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount,
        } => {
            let is_pool_exists = pools
                .get_pool(base_token_id.clone(), quote_token_id.clone())
                .await
                .is_some();

            if is_pool_exists {
                return Err(());
            }

            let mut base_token_balance = balances
                .get_balance(address.clone(), base_token_id.clone())
                .await
                .ok_or(())?;

            let mut quote_token_balance = balances
                .get_balance(address.clone(), quote_token_id.clone())
                .await
                .ok_or(())?;

            let (pool, liquidity) = nacho_executor::create_pool(
                &mut base_token_balance,
                &mut quote_token_balance,
                base_token_amount,
                quote_token_amount,
            )
            .map_err(|_| ())?;

            balances
                .update_balance(base_token_balance)
                .await
                .ok_or(())?;

            balances
                .update_balance(quote_token_balance)
                .await
                .ok_or(())?;

            liquidities.push_liquidity(liquidity).await.ok_or(())?;
            pools.push_pool(pool).await.ok_or(())?;
        }
        Transaction::ProvideLiquidity {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount_limit,
        } => {
            let mut base_token_balance = balances
                .get_balance(address.clone(), base_token_id.clone())
                .await
                .ok_or(())?;

            let mut quote_token_balance = balances
                .get_balance(address.clone(), quote_token_id.clone())
                .await
                .ok_or(())?;

            let mut pool = pools
                .get_pool(base_token_id.clone(), quote_token_id.clone())
                .await
                .ok_or(())?;

            let mut maybe_liquidity = liquidities
                .get_liquidity(
                    address.clone(),
                    base_token_id.clone(),
                    quote_token_id.clone(),
                )
                .await;

            let result = nacho_executor::provide_liquidity(
                &mut base_token_balance,
                &mut quote_token_balance,
                maybe_liquidity.as_mut(),
                &mut pool,
                base_token_amount,
                quote_token_amount_limit,
            )
            .map_err(|_| ())?;

            balances
                .update_balance(base_token_balance)
                .await
                .ok_or(())?;

            balances
                .update_balance(quote_token_balance)
                .await
                .ok_or(())?;

            pools.update_pool(pool).await.ok_or(())?;

            match (result, maybe_liquidity) {
                (Some(liquidity), None) => {
                    liquidities.push_liquidity(liquidity).await.ok_or(())?;
                }
                (None, Some(liquidity)) => {
                    liquidities.update_liquidity(liquidity).await.ok_or(())?;
                }
                _ => return Err(()),
            }
        }
        Transaction::RemoveLiquidity {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount_limit,
            quote_token_amount_limit,
            points,
        } => {
            let mut base_token_balance = balances
                .get_balance(address.clone(), base_token_id.clone())
                .await
                .ok_or(())?;

            let mut quote_token_balance = balances
                .get_balance(address.clone(), quote_token_id.clone())
                .await
                .ok_or(())?;

            let mut pool = pools
                .get_pool(base_token_id.clone(), quote_token_id.clone())
                .await
                .ok_or(())?;

            let mut liquidity = liquidities
                .get_liquidity(
                    address.clone(),
                    base_token_id.clone(),
                    quote_token_id.clone(),
                )
                .await
                .ok_or(())?;

            nacho_executor::remove_liquidity(
                &mut base_token_balance,
                &mut quote_token_balance,
                &mut liquidity,
                &mut pool,
                points,
                base_token_amount_limit,
                quote_token_amount_limit,
            )
            .map_err(|_| ())?;

            balances
                .update_balance(base_token_balance)
                .await
                .ok_or(())?;

            balances
                .update_balance(quote_token_balance)
                .await
                .ok_or(())?;

            pools.update_pool(pool).await.ok_or(())?;

            liquidities.update_liquidity(liquidity).await.ok_or(())?;
        }
        Transaction::BuyTokens {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount_limit,
        } => {
            let mut maybe_base_token_balance = balances
                .get_balance(address.clone(), base_token_id.clone())
                .await;

            let mut quote_token_balance = balances
                .get_balance(address.clone(), quote_token_id.clone())
                .await
                .ok_or(())?;

            let mut pool = pools
                .get_pool(base_token_id.clone(), quote_token_id.clone())
                .await
                .ok_or(())?;

            let result = nacho_executor::buy_tokens(
                base_token_id,
                maybe_base_token_balance.as_mut(),
                &mut quote_token_balance,
                &mut pool,
                base_token_amount,
                quote_token_amount_limit,
            )
            .map_err(|_| ())?;

            balances
                .update_balance(quote_token_balance)
                .await
                .ok_or(())?;

            pools.update_pool(pool).await.ok_or(())?;

            match (result, maybe_base_token_balance) {
                (Some(base_token_balance), None) => {
                    balances.push_balance(base_token_balance).await.ok_or(())?;
                }
                (None, Some(base_token_balance)) => {
                    balances
                        .update_balance(base_token_balance)
                        .await
                        .ok_or(())?;
                }
                _ => return Err(()),
            }
        }
        Transaction::SellTokens {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount_limit,
            quote_token_amount,
        } => {
            let mut base_token_balance = balances
                .get_balance(address.clone(), base_token_id.clone())
                .await
                .ok_or(())?;

            let mut maybe_quote_token_balance = balances
                .get_balance(address.clone(), quote_token_id.clone())
                .await;

            let mut pool = pools
                .get_pool(base_token_id.clone(), quote_token_id.clone())
                .await
                .ok_or(())?;

            let result = nacho_executor::sell_tokens(
                base_token_id,
                &mut base_token_balance,
                maybe_quote_token_balance.as_mut(),
                &mut pool,
                base_token_amount_limit,
                quote_token_amount,
            )
            .map_err(|_| ())?;

            balances
                .update_balance(base_token_balance)
                .await
                .ok_or(())?;

            pools.update_pool(pool).await.ok_or(())?;

            match (result, maybe_quote_token_balance) {
                (Some(quote_token_balance), None) => {
                    balances.push_balance(quote_token_balance).await.ok_or(())?;
                }
                (None, Some(quote_token_balance)) => {
                    balances
                        .update_balance(quote_token_balance)
                        .await
                        .ok_or(())?;
                }
                _ => return Err(()),
            }
        }
    }

    proofpool.push(tx).await.ok_or(())?;

    Ok(())
}
