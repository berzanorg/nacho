use super::Processor;
use crate::{balances, burns, liquidities, pools, proofpool, transactions};
use nacho_data_structures::{
    Balance, Burn, ByteConversion, Deposit, FieldConversion, Liquidity, Pool, ProverMethod,
    StateRoots, StatefulTransaction,
};
use nacho_poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};
use tokio::{
    process::{ChildStdin, ChildStdout},
    sync::Notify,
};

pub fn process(
    path: &str,
    transactions: transactions::Processor,
    proofpool: proofpool::Processor,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
) -> Processor {
    let notify: &Notify = Box::leak(Box::new(Notify::new()));

    let (stdin, stdout) = nacho_js_process::spawn(&[path]).unwrap();

    tokio::spawn(async move {
        let mut hasher = create_poseidon_hasher();

        loop {
            if let Some(stateful_tx) = proofpool.pop().await {
                generate_proof(
                    &stateful_tx,
                    stdin,
                    stdout,
                    balances,
                    pools,
                    liquidities,
                    burns,
                    transactions,
                    &mut hasher,
                )
                .await
                .ok();

                update_trees(stateful_tx, balances, pools, liquidities, burns)
                    .await
                    .ok();

                if let Some(proved_until) = transactions.get_proved_until().await {
                    transactions.set_proved_until(proved_until + 1).await;
                }
            } else {
                notify.notified().await;
            }
        }
    });

    Processor { notify }
}

pub async fn generate_proof(
    stateful_tx: &StatefulTransaction,
    stdin: &mut ChildStdin,
    stdout: &mut ChildStdout,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
    transactions: transactions::Processor,
    hasher: &mut PoseidonHasher,
) -> Result<(), ()> {
    let prover_method = create_prover_method(
        stateful_tx,
        balances,
        pools,
        liquidities,
        burns,
        transactions,
        hasher,
    )
    .await
    .ok_or(())?;

    let input = prover_method.to_bytes();
    let mut output = [0; 1];

    nacho_js_process::interact(stdin, stdout, &input, &mut output)
        .await
        .map_err(|_| ())?;

    let is_success = output[0] != 0;

    if is_success {
        Ok(())
    } else {
        Err(())
    }
}

pub async fn create_prover_method(
    stateful_tx: &StatefulTransaction,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
    transactions: transactions::Processor,
    hasher: &mut PoseidonHasher,
) -> Option<ProverMethod> {
    Some(match stateful_tx {
        StatefulTransaction::CreateGenesis {
            transaction: (),
            state: (),
        } => ProverMethod::CreateGenesis {
            state_roots: StateRoots {
                balances: balances.get_root().await?,
                liquidities: liquidities.get_root().await?,
                pools: pools.get_root().await?,
                burns: burns.get_root().await?,
            },
        },

        StatefulTransaction::DepositTokens { transaction, state } => ProverMethod::DepositTokens {
            state_roots: StateRoots {
                balances: balances.get_root().await?,
                liquidities: liquidities.get_root().await?,
                pools: pools.get_root().await?,
                burns: burns.get_root().await?,
            },
            earlier_proof_index: transactions.get_proved_until().await? - 1,
            single_balance_witness: balances
                .get_single_witness(
                    transaction.user_address.clone(),
                    transaction.token_id.clone(),
                )
                .await
                .unwrap_or(balances.get_new_witness().await?),
            // Below is just for testing. It shouldn't be used for production.
            current_deposits_merkle_list_hash: 0u64.into(),
            expected_deposits_merkle_list_hash: {
                let deposit_hash = poseidon_hash(
                    hasher,
                    &Deposit {
                        depositor: transaction.user_address.clone(),
                        token_id: transaction.token_id.clone(),
                        token_amount: transaction.token_amount,
                    }
                    .to_fields(),
                );
                poseidon_hash(hasher, &[0.into(), deposit_hash]).into()
            },
            user_address: transaction.user_address.clone(),
            token_id: transaction.token_id.clone(),
            user_deposit_token_amount: transaction.token_amount,
            user_balance_token_amount: state.user_token_balance,
        },

        StatefulTransaction::BurnTokens { transaction, state } => ProverMethod::BurnTokens {
            state_roots: StateRoots {
                balances: balances.get_root().await?,
                liquidities: liquidities.get_root().await?,
                pools: pools.get_root().await?,
                burns: burns.get_root().await?,
            },
            earlier_proof_index: transactions.get_proved_until().await? - 1,
            single_balance_witness: balances
                .get_single_witness(transaction.address.clone(), transaction.token_id.clone())
                .await?,
            single_burn_witness: burns
                .get_witness(transaction.address.clone(), transaction.token_id.clone())
                .await
                .map(|(witness, _)| witness)
                .unwrap_or(burns.get_new_witness().await?),
            user_address: transaction.address.clone(),
            token_id: transaction.token_id.clone(),
            user_burn_token_amount: state.user_burn_token_amount,
            user_balance_token_amount: state.user_balance_token_amount,
            amount_to_burn: transaction.token_amount,
            user_signature: transaction.signature.clone(),
        },

        StatefulTransaction::CreatePool { transaction, state } => ProverMethod::CreatePool {
            state_roots: StateRoots {
                balances: balances.get_root().await?,
                liquidities: liquidities.get_root().await?,
                pools: pools.get_root().await?,
                burns: burns.get_root().await?,
            },
            earlier_proof_index: transactions.get_proved_until().await? - 1,
            single_pool_witness: pools.get_new_witness().await?,
            single_liquidity_witness: liquidities.get_new_witness().await?,
            double_balance_witness: balances
                .get_double_witness(
                    transaction.address.clone(),
                    transaction.base_token_id.clone(),
                    transaction.quote_token_id.clone(),
                )
                .await?,
            base_token_id: transaction.base_token_id.clone(),
            quote_token_id: transaction.quote_token_id.clone(),
            user_address: transaction.address.clone(),
            user_liquidity_base_token_amount: transaction.base_token_amount,
            user_liquidity_quote_token_amount: transaction.quote_token_amount,
            user_balance_base_token_amount: state.user_balance_base_token_amount,
            user_balance_quote_token_amount: state.user_balance_quote_token_amount,
            user_signature: transaction.signature.clone(),
        },

        StatefulTransaction::ProvideLiquidity { transaction, state } => {
            ProverMethod::ProvideLiquidity {
                state_roots: StateRoots {
                    balances: balances.get_root().await?,
                    liquidities: liquidities.get_root().await?,
                    pools: pools.get_root().await?,
                    burns: burns.get_root().await?,
                },
                earlier_proof_index: transactions.get_proved_until().await? - 1,
                single_pool_witness: pools
                    .get_witness(
                        transaction.base_token_id.clone(),
                        transaction.quote_token_id.clone(),
                    )
                    .await?,
                single_liquidity_witness: liquidities
                    .get_witness(
                        transaction.address.clone(),
                        transaction.base_token_id.clone(),
                        transaction.quote_token_id.clone(),
                    )
                    .await
                    .unwrap_or(liquidities.get_new_witness().await?),
                double_balance_witness: balances
                    .get_double_witness(
                        transaction.address.clone(),
                        transaction.base_token_id.clone(),
                        transaction.quote_token_id.clone(),
                    )
                    .await?,
                base_token_id: transaction.base_token_id.clone(),
                quote_token_id: transaction.quote_token_id.clone(),
                user_address: transaction.address.clone(),
                user_liquidity_points: state.user_liquidity_points.clone(),
                user_balance_base_token_amount: state.user_balance_base_token_amount,
                user_balance_quote_token_amount: state.user_balance_quote_token_amount,
                pool_base_token_amount: state.pool_base_token_amount,
                pool_quote_token_amount: state.pool_quote_token_amount,
                pool_total_liquidity_points: state.pool_total_liquidity_points.clone(),
                user_base_token_amount_to_provide: transaction.base_token_amount,
                user_quote_token_amount_limit_to_provide: transaction.quote_token_amount_limit,
                user_signature: transaction.signature.clone(),
            }
        }

        StatefulTransaction::RemoveLiquidity { transaction, state } => {
            ProverMethod::RemoveLiquidity {
                state_roots: StateRoots {
                    balances: balances.get_root().await?,
                    liquidities: liquidities.get_root().await?,
                    pools: pools.get_root().await?,
                    burns: burns.get_root().await?,
                },
                earlier_proof_index: transactions.get_proved_until().await? - 1,
                single_pool_witness: pools
                    .get_witness(
                        transaction.base_token_id.clone(),
                        transaction.quote_token_id.clone(),
                    )
                    .await?,
                single_liquidity_witness: liquidities
                    .get_witness(
                        transaction.address.clone(),
                        transaction.base_token_id.clone(),
                        transaction.quote_token_id.clone(),
                    )
                    .await?,
                double_balance_witness: balances
                    .get_double_witness(
                        transaction.address.clone(),
                        transaction.base_token_id.clone(),
                        transaction.quote_token_id.clone(),
                    )
                    .await?,
                base_token_id: transaction.base_token_id.clone(),
                quote_token_id: transaction.quote_token_id.clone(),
                user_address: transaction.address.clone(),
                user_liquidity_points: state.user_liquidity_points.clone(),
                user_balance_base_token_amount: state.user_balance_base_token_amount,
                user_balance_quote_token_amount: state.user_balance_quote_token_amount,
                pool_base_token_amount: state.pool_base_token_amount,
                pool_quote_token_amount: state.pool_quote_token_amount,
                pool_total_liquidity_points: state.pool_total_liquidity_points.clone(),
                user_liquidity_points_to_remove: transaction.points.clone(),
                user_base_token_amount_limit_to_remove: transaction.base_token_amount_limit,
                user_quote_token_amount_limit_to_remove: transaction.quote_token_amount_limit,
                user_signature: transaction.signature.clone(),
            }
        }

        StatefulTransaction::BuyTokens { transaction, state } => ProverMethod::BuyTokens {
            state_roots: StateRoots {
                balances: balances.get_root().await?,
                liquidities: liquidities.get_root().await?,
                pools: pools.get_root().await?,
                burns: burns.get_root().await?,
            },
            earlier_proof_index: transactions.get_proved_until().await? - 1,
            single_pool_witness: pools
                .get_witness(
                    transaction.base_token_id.clone(),
                    transaction.quote_token_id.clone(),
                )
                .await?,
            double_balance_witness: (
                balances
                    .get_single_witness(
                        transaction.address.clone(),
                        transaction.base_token_id.clone(),
                    )
                    .await
                    .unwrap_or(balances.get_new_witness().await?),
                balances
                    .get_single_witness(
                        transaction.address.clone(),
                        transaction.quote_token_id.clone(),
                    )
                    .await?,
            )
                .into(),
            user_address: transaction.address.clone(),
            base_token_id: transaction.base_token_id.clone(),
            quote_token_id: transaction.quote_token_id.clone(),
            user_balance_base_token_amount: state.user_balance_base_token_amount,
            user_balance_quote_token_amount: state.user_balance_quote_token_amount,
            pool_base_token_amount: state.pool_base_token_amount,
            pool_quote_token_amount: state.pool_quote_token_amount,
            pool_total_liquidity_points: state.pool_total_liquidity_points.clone(),
            user_base_token_amount_to_swap: transaction.base_token_amount,
            user_quote_token_amount_limit_to_swap: transaction.quote_token_amount_limit,
            user_signature: transaction.signature.clone(),
        },

        StatefulTransaction::SellTokens { transaction, state } => ProverMethod::SellTokens {
            state_roots: StateRoots {
                balances: balances.get_root().await?,
                liquidities: liquidities.get_root().await?,
                pools: pools.get_root().await?,
                burns: burns.get_root().await?,
            },
            earlier_proof_index: transactions.get_proved_until().await? - 1,
            single_pool_witness: pools
                .get_witness(
                    transaction.base_token_id.clone(),
                    transaction.quote_token_id.clone(),
                )
                .await?,
            double_balance_witness: (
                balances
                    .get_single_witness(
                        transaction.address.clone(),
                        transaction.base_token_id.clone(),
                    )
                    .await?,
                balances
                    .get_single_witness(
                        transaction.address.clone(),
                        transaction.quote_token_id.clone(),
                    )
                    .await
                    .unwrap_or(balances.get_new_witness().await?),
            )
                .into(),
            user_address: transaction.address.clone(),
            base_token_id: transaction.base_token_id.clone(),
            quote_token_id: transaction.quote_token_id.clone(),
            user_balance_base_token_amount: state.user_balance_base_token_amount,
            user_balance_quote_token_amount: state.user_balance_quote_token_amount,
            pool_base_token_amount: state.pool_base_token_amount,
            pool_quote_token_amount: state.pool_quote_token_amount,
            pool_total_liquidity_points: state.pool_total_liquidity_points.clone(),
            user_base_token_amount_limit_to_swap: transaction.base_token_amount_limit,
            user_quote_token_amount_to_swap: transaction.quote_token_amount,
            user_signature: transaction.signature.clone(),
        },
    })
}

pub async fn update_trees(
    stateful_tx: StatefulTransaction,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
) -> Result<(), ()> {
    match stateful_tx {
        StatefulTransaction::CreateGenesis { transaction, state } => {
            let _ = (transaction, state);
        }
        StatefulTransaction::DepositTokens { transaction, state } => {
            let mut maybe_balance = if balances
                .get_single_witness(
                    transaction.user_address.clone(),
                    transaction.token_id.clone(),
                )
                .await
                .is_some()
            {
                Some(Balance {
                    owner: transaction.user_address.clone(),
                    token_amount: state.user_token_balance,
                    token_id: transaction.token_id.clone(),
                })
            } else {
                None
            };

            let amount_to_deposit = transaction.token_amount;

            let result = nacho_executor::deposit_tokens(
                maybe_balance.as_mut(),
                amount_to_deposit,
                transaction.token_id.clone(),
                transaction.user_address.clone(),
            )
            .map_err(|_| ())?;

            match (result, maybe_balance) {
                (Some(balance), None) => {
                    balances.push_leaf(balance).await.ok_or(())?;
                }
                (None, Some(balance)) => {
                    balances.update_leaf(balance).await.ok_or(())?;
                }
                _ => return Err(()),
            }
        }
        StatefulTransaction::BurnTokens { transaction, state } => {
            let mut balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.token_id.clone(),
                token_amount: state.user_balance_token_amount,
            };

            let mut maybe_burn = if burns
                .get_witness(transaction.address.clone(), transaction.token_id.clone())
                .await
                .is_some()
            {
                Some(Burn {
                    burner: transaction.address.clone(),
                    token_id: transaction.token_id.clone(),
                    token_amount: state.user_burn_token_amount,
                })
            } else {
                None
            };

            let amount_to_burn = transaction.token_amount;

            let result =
                nacho_executor::burn_tokens(&mut balance, maybe_burn.as_mut(), amount_to_burn)
                    .map_err(|_| ())?;

            balances.update_leaf(balance).await.ok_or(())?;

            match (result, maybe_burn) {
                (Some(burn), None) => {
                    burns.push_leaf(burn).await.ok_or(())?;
                }
                (None, Some(burn)) => {
                    burns.update_leaf(burn).await.ok_or(())?;
                }
                _ => return Err(()),
            }
        }
        StatefulTransaction::CreatePool { transaction, state } => {
            let mut base_token_balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.base_token_id.clone(),
                token_amount: state.user_balance_base_token_amount,
            };

            let mut quote_token_balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.quote_token_id.clone(),
                token_amount: state.user_balance_quote_token_amount,
            };

            let (pool, liquidity) = nacho_executor::create_pool(
                &mut base_token_balance,
                &mut quote_token_balance,
                transaction.base_token_amount,
                transaction.quote_token_amount,
            )
            .map_err(|_| ())?;

            balances.update_leaf(base_token_balance).await.ok_or(())?;

            balances.update_leaf(quote_token_balance).await.ok_or(())?;

            liquidities.push_leaf(liquidity).await.ok_or(())?;
            pools.push_leaf(pool).await.ok_or(())?;
        }
        StatefulTransaction::ProvideLiquidity { transaction, state } => {
            let mut base_token_balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.base_token_id.clone(),
                token_amount: state.user_balance_base_token_amount,
            };

            let mut quote_token_balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.quote_token_id.clone(),
                token_amount: state.user_balance_quote_token_amount,
            };

            let mut pool = Pool {
                base_token_id: transaction.base_token_id.clone(),
                quote_token_id: transaction.quote_token_id.clone(),
                base_token_amount: state.pool_base_token_amount,
                quote_token_amount: state.pool_quote_token_amount,
                total_liqudity_points: state.pool_total_liquidity_points.clone(),
            };

            let mut maybe_liquidity = if liquidities
                .get_witness(
                    transaction.address.clone(),
                    transaction.base_token_id.clone(),
                    transaction.quote_token_id.clone(),
                )
                .await
                .is_some()
            {
                Some(Liquidity {
                    provider: transaction.address.clone(),
                    base_token_id: transaction.base_token_id.clone(),
                    quote_token_id: transaction.quote_token_id.clone(),
                    points: state.user_liquidity_points.clone(),
                })
            } else {
                None
            };

            let result = nacho_executor::provide_liquidity(
                &mut base_token_balance,
                &mut quote_token_balance,
                maybe_liquidity.as_mut(),
                &mut pool,
                transaction.base_token_amount,
                transaction.quote_token_amount_limit,
            )
            .map_err(|_| ())?;

            balances.update_leaf(base_token_balance).await.ok_or(())?;

            balances.update_leaf(quote_token_balance).await.ok_or(())?;

            pools.update_leaf(pool).await.ok_or(())?;

            match (result, maybe_liquidity) {
                (Some(liquidity), None) => {
                    liquidities.push_leaf(liquidity).await.ok_or(())?;
                }
                (None, Some(liquidity)) => {
                    liquidities.update_leaf(liquidity).await.ok_or(())?;
                }
                _ => return Err(()),
            }
        }
        StatefulTransaction::RemoveLiquidity { transaction, state } => {
            let mut base_token_balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.base_token_id.clone(),
                token_amount: state.user_balance_base_token_amount,
            };

            let mut quote_token_balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.quote_token_id.clone(),
                token_amount: state.user_balance_quote_token_amount,
            };

            let mut pool = Pool {
                base_token_id: transaction.base_token_id.clone(),
                quote_token_id: transaction.quote_token_id.clone(),
                base_token_amount: state.pool_base_token_amount,
                quote_token_amount: state.pool_quote_token_amount,
                total_liqudity_points: state.pool_total_liquidity_points.clone(),
            };

            let mut liquidity = Liquidity {
                provider: transaction.address.clone(),
                base_token_id: transaction.base_token_id.clone(),
                quote_token_id: transaction.quote_token_id.clone(),
                points: state.user_liquidity_points.clone(),
            };

            nacho_executor::remove_liquidity(
                &mut base_token_balance,
                &mut quote_token_balance,
                &mut liquidity,
                &mut pool,
                transaction.points.clone(),
                transaction.base_token_amount_limit,
                transaction.quote_token_amount_limit,
            )
            .map_err(|_| ())?;

            balances.update_leaf(base_token_balance).await.ok_or(())?;

            balances.update_leaf(quote_token_balance).await.ok_or(())?;

            pools.update_leaf(pool).await.ok_or(())?;

            liquidities.update_leaf(liquidity).await.ok_or(())?;
        }
        StatefulTransaction::BuyTokens { transaction, state } => {
            let mut maybe_base_token_balance = if balances
                .get_single_witness(
                    transaction.address.clone(),
                    transaction.base_token_id.clone(),
                )
                .await
                .is_some()
            {
                Some(Balance {
                    owner: transaction.address.clone(),
                    token_amount: state.user_balance_base_token_amount,
                    token_id: transaction.base_token_id.clone(),
                })
            } else {
                None
            };

            let mut quote_token_balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.quote_token_id.clone(),
                token_amount: state.user_balance_quote_token_amount,
            };

            let mut pool = Pool {
                base_token_id: transaction.base_token_id.clone(),
                quote_token_id: transaction.quote_token_id.clone(),
                base_token_amount: state.pool_base_token_amount,
                quote_token_amount: state.pool_quote_token_amount,
                total_liqudity_points: state.pool_total_liquidity_points.clone(),
            };

            let result = nacho_executor::buy_tokens(
                transaction.base_token_id.clone(),
                maybe_base_token_balance.as_mut(),
                &mut quote_token_balance,
                &mut pool,
                transaction.base_token_amount,
                transaction.quote_token_amount_limit,
            )
            .map_err(|_| ())?;

            balances.update_leaf(quote_token_balance).await.ok_or(())?;

            pools.update_leaf(pool).await.ok_or(())?;

            match (result, maybe_base_token_balance) {
                (Some(base_token_balance), None) => {
                    balances.push_leaf(base_token_balance).await.ok_or(())?;
                }
                (None, Some(base_token_balance)) => {
                    balances.update_leaf(base_token_balance).await.ok_or(())?;
                }
                _ => return Err(()),
            }
        }
        StatefulTransaction::SellTokens { transaction, state } => {
            let mut base_token_balance = Balance {
                owner: transaction.address.clone(),
                token_id: transaction.base_token_id.clone(),
                token_amount: state.user_balance_base_token_amount,
            };

            let mut maybe_quote_token_balance = if balances
                .get_single_witness(
                    transaction.address.clone(),
                    transaction.quote_token_id.clone(),
                )
                .await
                .is_some()
            {
                Some(Balance {
                    owner: transaction.address.clone(),
                    token_amount: state.user_balance_quote_token_amount,
                    token_id: transaction.quote_token_id.clone(),
                })
            } else {
                None
            };

            let mut pool = Pool {
                base_token_id: transaction.base_token_id.clone(),
                quote_token_id: transaction.quote_token_id.clone(),
                base_token_amount: state.pool_base_token_amount,
                quote_token_amount: state.pool_quote_token_amount,
                total_liqudity_points: state.pool_total_liquidity_points.clone(),
            };

            let result = nacho_executor::sell_tokens(
                transaction.base_token_id,
                &mut base_token_balance,
                maybe_quote_token_balance.as_mut(),
                &mut pool,
                transaction.base_token_amount_limit,
                transaction.quote_token_amount,
            )
            .map_err(|_| ())?;

            balances.update_leaf(base_token_balance).await.ok_or(())?;

            pools.update_leaf(pool).await.ok_or(())?;

            match (result, maybe_quote_token_balance) {
                (Some(quote_token_balance), None) => {
                    balances.push_leaf(quote_token_balance).await.ok_or(())?;
                }
                (None, Some(quote_token_balance)) => {
                    balances.update_leaf(quote_token_balance).await.ok_or(())?;
                }
                _ => return Err(()),
            }
        }
    }

    Ok(())
}
