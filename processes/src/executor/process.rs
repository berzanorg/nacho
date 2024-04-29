use super::Processor;
use crate::{balances, burns, liquidities, mempool, pools, proofpool, transactions, verifier};
use nacho_data_structures::{
    BurnTokensTransaction, BurnTokensTransactionState, BuyTokensTransaction,
    BuyTokensTransactionState, CreatePoolTransaction, CreatePoolTransactionState,
    DepositTokensTransaction, DepositTokensTransactionState, ProvideLiquidityTransaction,
    ProvideLiquidityTransactionState, RemoveLiquidityTransaction, RemoveLiquidityTransactionState,
    SellTokensTransaction, SellTokensTransactionState, StatefulTransaction, Transaction,
};
use tokio::sync::Notify;

pub fn process(
    verifier: verifier::Processor,
    transactions: transactions::Processor,
    mempool: mempool::Processor,
    proofpool: proofpool::Processor,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
) -> Processor {
    let notify: &Notify = Box::leak(Box::new(Notify::new()));

    tokio::spawn(async move {
        loop {
            if let Some(tx) = mempool.pop().await {
                let tx_result =
                    execute_tx(tx, verifier, proofpool, balances, pools, liquidities, burns).await;

                if tx_result.is_err() {
                    if let Some(executed_until) = transactions.get_executed_until().await {
                        transactions.set_rejected(executed_until).await;
                    };
                } else {
                    if let Some(executed_until) = transactions.get_executed_until().await {
                        transactions.set_executed_until(executed_until + 1).await;
                    };
                }
            } else {
                notify.notified().await;
            }
        }
    });

    Processor { notify }
}

pub async fn execute_tx(
    tx: Transaction,
    verifier: verifier::Processor,
    proofpool: proofpool::Processor,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
) -> Result<(), ()> {
    let is_valid = verifier.check_signature(tx.clone()).await.ok_or(())?;

    if !is_valid {
        return Err(());
    }

    match tx.clone() {
        Transaction::CreateGenesis {} => {}
        Transaction::DepositTokens(DepositTokensTransaction {
            user_address,
            token_id,
            token_amount,
        }) => {
            let mut maybe_balance = balances
                .get_balance(user_address.clone(), token_id.clone())
                .await;

            let amount_to_deposit = token_amount;

            let result = nacho_executor::deposit_tokens(
                maybe_balance.as_mut(),
                amount_to_deposit,
                token_id,
                user_address,
            )
            .map_err(|_| ())?;

            match (result, maybe_balance) {
                (Some(balance), None) => {
                    balances.push_balance(balance).await.ok_or(())?;
                }
                (None, Some(balance)) => {
                    balances.update_balance(balance).await.ok_or(())?;
                }
                _ => return Err(()),
            }
        }
        Transaction::BurnTokens(BurnTokensTransaction {
            address,
            signature: _,
            token_id,
            token_amount,
        }) => {
            let mut balance = balances
                .get_balance(address.clone(), token_id.clone())
                .await
                .ok_or(())?;

            let mut maybe_burn = burns.get_burn(address.clone(), token_id.clone()).await;

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
        Transaction::CreatePool(CreatePoolTransaction {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount,
        }) => {
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
        Transaction::ProvideLiquidity(ProvideLiquidityTransaction {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount_limit,
        }) => {
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
        Transaction::RemoveLiquidity(RemoveLiquidityTransaction {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount_limit,
            quote_token_amount_limit,
            points,
        }) => {
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
        Transaction::BuyTokens(BuyTokensTransaction {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount_limit,
        }) => {
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
        Transaction::SellTokens(SellTokensTransaction {
            address,
            signature: _,
            base_token_id,
            quote_token_id,
            base_token_amount_limit,
            quote_token_amount,
        }) => {
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

    proofpool
        .push(match tx {
            Transaction::CreateGenesis() => StatefulTransaction::CreateGenesis {
                transaction: (),
                state: (),
            },
            Transaction::DepositTokens(deposit_token_tx) => {
                let maybe_balance = balances
                    .get_balance(
                        deposit_token_tx.user_address.clone(),
                        deposit_token_tx.token_id.clone(),
                    )
                    .await;

                StatefulTransaction::DepositTokens {
                    transaction: deposit_token_tx,
                    state: DepositTokensTransactionState {
                        user_token_balance: maybe_balance
                            .map(|balance| balance.token_amount)
                            .unwrap_or(0),
                    },
                }
            }
            Transaction::BurnTokens(burn_tokens_tx) => {
                let balance = balances
                    .get_balance(
                        burn_tokens_tx.address.clone(),
                        burn_tokens_tx.token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let maybe_burn = burns
                    .get_burn(
                        burn_tokens_tx.address.clone(),
                        burn_tokens_tx.token_id.clone(),
                    )
                    .await;

                StatefulTransaction::BurnTokens {
                    transaction: burn_tokens_tx,
                    state: BurnTokensTransactionState {
                        user_balance_token_amount: balance.token_amount,
                        user_burn_token_amount: maybe_burn
                            .map(|burn| burn.token_amount)
                            .unwrap_or(0),
                    },
                }
            }
            Transaction::CreatePool(create_pool_tx) => {
                let base_token_balance = balances
                    .get_balance(
                        create_pool_tx.address.clone(),
                        create_pool_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let quote_token_balance = balances
                    .get_balance(
                        create_pool_tx.address.clone(),
                        create_pool_tx.quote_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                StatefulTransaction::CreatePool {
                    transaction: create_pool_tx,
                    state: CreatePoolTransactionState {
                        user_balance_base_token_amount: base_token_balance.token_amount,
                        user_balance_quote_token_amount: quote_token_balance.token_amount,
                    },
                }
            }
            Transaction::ProvideLiquidity(provide_liquidity_tx) => {
                let maybe_liquidity = liquidities
                    .get_liquidity(
                        provide_liquidity_tx.address.clone(),
                        provide_liquidity_tx.base_token_id.clone(),
                        provide_liquidity_tx.quote_token_id.clone(),
                    )
                    .await;

                let base_token_balance = balances
                    .get_balance(
                        provide_liquidity_tx.address.clone(),
                        provide_liquidity_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let quote_token_balance = balances
                    .get_balance(
                        provide_liquidity_tx.address.clone(),
                        provide_liquidity_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let pool = pools
                    .get_pool(
                        provide_liquidity_tx.base_token_id.clone(),
                        provide_liquidity_tx.quote_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                StatefulTransaction::ProvideLiquidity {
                    transaction: provide_liquidity_tx,
                    state: ProvideLiquidityTransactionState {
                        user_liquidity_points: maybe_liquidity
                            .map(|liquidity| liquidity.points)
                            .unwrap_or(0u64.into()),
                        user_balance_base_token_amount: base_token_balance.token_amount,
                        user_balance_quote_token_amount: quote_token_balance.token_amount,
                        pool_base_token_amount: pool.base_token_amount,
                        pool_quote_token_amount: pool.quote_token_amount,
                        pool_total_liquidity_points: pool.total_liqudity_points,
                    },
                }
            }
            Transaction::RemoveLiquidity(remove_liquidity_tx) => {
                let liquidity = liquidities
                    .get_liquidity(
                        remove_liquidity_tx.address.clone(),
                        remove_liquidity_tx.base_token_id.clone(),
                        remove_liquidity_tx.quote_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let base_token_balance = balances
                    .get_balance(
                        remove_liquidity_tx.address.clone(),
                        remove_liquidity_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let quote_token_balance = balances
                    .get_balance(
                        remove_liquidity_tx.address.clone(),
                        remove_liquidity_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let pool = pools
                    .get_pool(
                        remove_liquidity_tx.base_token_id.clone(),
                        remove_liquidity_tx.quote_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                StatefulTransaction::RemoveLiquidity {
                    transaction: remove_liquidity_tx,
                    state: RemoveLiquidityTransactionState {
                        user_liquidity_points: liquidity.points,
                        user_balance_base_token_amount: base_token_balance.token_amount,
                        user_balance_quote_token_amount: quote_token_balance.token_amount,
                        pool_base_token_amount: pool.base_token_amount,
                        pool_quote_token_amount: pool.quote_token_amount,
                        pool_total_liquidity_points: pool.total_liqudity_points,
                    },
                }
            }
            Transaction::BuyTokens(buy_tokens_tx) => {
                let base_token_balance = balances
                    .get_balance(
                        buy_tokens_tx.address.clone(),
                        buy_tokens_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let quote_token_balance = balances
                    .get_balance(
                        buy_tokens_tx.address.clone(),
                        buy_tokens_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let pool = pools
                    .get_pool(
                        buy_tokens_tx.base_token_id.clone(),
                        buy_tokens_tx.quote_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                StatefulTransaction::BuyTokens {
                    transaction: buy_tokens_tx,
                    state: BuyTokensTransactionState {
                        user_balance_base_token_amount: base_token_balance.token_amount,
                        user_balance_quote_token_amount: quote_token_balance.token_amount,
                        pool_base_token_amount: pool.base_token_amount,
                        pool_quote_token_amount: pool.quote_token_amount,
                        pool_total_liquidity_points: pool.total_liqudity_points,
                    },
                }
            }
            Transaction::SellTokens(sell_tokens_tx) => {
                let base_token_balance = balances
                    .get_balance(
                        sell_tokens_tx.address.clone(),
                        sell_tokens_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let quote_token_balance = balances
                    .get_balance(
                        sell_tokens_tx.address.clone(),
                        sell_tokens_tx.base_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                let pool = pools
                    .get_pool(
                        sell_tokens_tx.base_token_id.clone(),
                        sell_tokens_tx.quote_token_id.clone(),
                    )
                    .await
                    .ok_or(())?;

                StatefulTransaction::SellTokens {
                    transaction: sell_tokens_tx,
                    state: SellTokensTransactionState {
                        user_balance_base_token_amount: base_token_balance.token_amount,
                        user_balance_quote_token_amount: quote_token_balance.token_amount,
                        pool_base_token_amount: pool.base_token_amount,
                        pool_quote_token_amount: pool.quote_token_amount,
                        pool_total_liquidity_points: pool.total_liqudity_points,
                    },
                }
            }
        })
        .await
        .ok_or(())?;

    Ok(())
}
