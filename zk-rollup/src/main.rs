use nacho_data_structures::{
    BurnTokensTransaction, BuyTokensTransaction, CreatePoolTransaction,
    ProvideLiquidityTransaction, RemoveLiquidityTransaction, SellTokensTransaction, Transaction,
};
use nacho_rpc_server::{start_rpc_server, RpcMethod::*, RpcResponse::*};

#[tokio::main]
async fn main() {
    let balances = nacho_processes::balances::process();
    let burns = nacho_processes::burns::process();
    let liquidities = nacho_processes::liquidities::process();
    let mempool = nacho_processes::mempool::process();
    let pools = nacho_processes::pools::process();
    let proofpool = nacho_processes::proofpool::process();
    let _submitter = nacho_processes::submitter::process();
    let transactions = nacho_processes::transactions::process();
    let verifier = nacho_processes::verifier::process();
    let withdrawals = nacho_processes::withdrawals::process();
    let _merger = nacho_processes::merger::process(transactions);
    let executor = nacho_processes::executor::process(
        balances,
        burns,
        liquidities,
        mempool,
        pools,
        proofpool,
        transactions,
        verifier,
    );
    let generator = nacho_processes::generator::process(
        balances,
        burns,
        liquidities,
        pools,
        proofpool,
        transactions,
    );
    let _fetcher = nacho_processes::fetcher::process(
        burns,
        executor,
        generator,
        mempool,
        transactions,
        withdrawals,
    );

    start_rpc_server(move |method| async move {
        match method {
            Unknown => ClientError,

            GetTotalTxCount => {
                let maybe_total_tx_count = transactions.get_total_tx_count().await;

                maybe_total_tx_count
                    .map(|total_tx_count| TotalTxCount(total_tx_count))
                    .unwrap_or(ServerError)
            }

            GetTxStatus { tx_id } => {
                let maybe_tx_status = transactions.get_tx_status(tx_id).await;

                maybe_tx_status
                    .map(|tx_status| TxStatus(tx_status))
                    .unwrap_or(ServerError)
            }

            GetBalances { address } => {
                let maybe_balances = balances.get_balances(address).await;

                maybe_balances
                    .map(|balances| {
                        Balances(
                            balances
                                .into_iter()
                                .map(|balance| (balance.token_id, balance.token_amount))
                                .collect::<_>(),
                        )
                    })
                    .unwrap_or(ServerError)
            }

            GetPools => {
                let maybe_pools = pools.get_pools().await;

                maybe_pools
                    .map(|pools| {
                        Pools(
                            pools
                                .into_iter()
                                .map(|pool| {
                                    (
                                        pool.base_token_id,
                                        pool.quote_token_id,
                                        pool.base_token_amount,
                                        pool.quote_token_amount,
                                        pool.total_liqudity_points,
                                    )
                                })
                                .collect::<_>(),
                        )
                    })
                    .unwrap_or(ServerError)
            }

            GetLiquidities { address } => {
                let maybe_liquidities = liquidities.get_liquidities(address).await;

                maybe_liquidities
                    .map(|liquidities| {
                        Liquidites(
                            liquidities
                                .into_iter()
                                .map(|liquidity| {
                                    (
                                        liquidity.base_token_id,
                                        liquidity.quote_token_id,
                                        liquidity.points,
                                    )
                                })
                                .collect::<_>(),
                        )
                    })
                    .unwrap_or(ServerError)
            }

            GetBurns { address } => {
                let maybe_liquidities = liquidities.get_liquidities(address).await;

                maybe_liquidities
                    .map(|liquidities| {
                        Liquidites(
                            liquidities
                                .into_iter()
                                .map(|liquidity| {
                                    (
                                        liquidity.base_token_id,
                                        liquidity.quote_token_id,
                                        liquidity.points,
                                    )
                                })
                                .collect::<_>(),
                        )
                    })
                    .unwrap_or(ServerError)
            }

            GetBridgeWitnesses { address, token_id } => {
                let (burn_witness, burn_index) = match burns.get_witness(address, token_id).await {
                    Some((burn_witness, burn_index)) => (burn_witness, burn_index),
                    None => return ServerError,
                };

                let withdrawal_witness = match withdrawals.get_witness(burn_index).await {
                    Some(withdrawal_witness) => withdrawal_witness,
                    None => return ServerError,
                };

                BridgeWitnesses(burn_witness, withdrawal_witness)
            }

            BurnTokens {
                address,
                signature,
                token_id,
                token_amount,
            } => {
                let tx_id = match transactions.add_new_tx().await {
                    Some(tx_id) => tx_id,
                    None => return ServerError,
                };

                let transaction = Transaction::BurnTokens(BurnTokensTransaction {
                    address,
                    signature,
                    token_id,
                    token_amount,
                });

                if mempool.push(transaction).await.is_none() {
                    return ServerError;
                };

                executor.keep_executing();
                generator.keep_generating();

                TxId(tx_id)
            }

            CreatePool {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount,
            } => {
                let tx_id = match transactions.add_new_tx().await {
                    Some(tx_id) => tx_id,
                    None => return ServerError,
                };

                let transaction = Transaction::CreatePool(CreatePoolTransaction {
                    address,
                    signature,
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount,
                });

                if mempool.push(transaction).await.is_none() {
                    return ServerError;
                };

                executor.keep_executing();
                generator.keep_generating();

                TxId(tx_id)
            }

            ProvideLiquidity {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount_limit,
            } => {
                let tx_id = match transactions.add_new_tx().await {
                    Some(tx_id) => tx_id,
                    None => return ServerError,
                };

                let transaction = Transaction::ProvideLiquidity(ProvideLiquidityTransaction {
                    address,
                    signature,
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount_limit,
                });

                if mempool.push(transaction).await.is_none() {
                    return ServerError;
                };

                executor.keep_executing();
                generator.keep_generating();

                TxId(tx_id)
            }

            RemoveLiquidity {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount_limit,
                points,
            } => {
                let tx_id = match transactions.add_new_tx().await {
                    Some(tx_id) => tx_id,
                    None => return ServerError,
                };

                let transaction = Transaction::RemoveLiquidity(RemoveLiquidityTransaction {
                    address,
                    signature,
                    base_token_id,
                    quote_token_id,
                    base_token_amount_limit,
                    quote_token_amount_limit,
                    points,
                });

                if mempool.push(transaction).await.is_none() {
                    return ServerError;
                };

                executor.keep_executing();
                generator.keep_generating();

                TxId(tx_id)
            }

            BuyTokens {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount_limit,
            } => {
                let tx_id = match transactions.add_new_tx().await {
                    Some(tx_id) => tx_id,
                    None => return ServerError,
                };

                let transaction = Transaction::BuyTokens(BuyTokensTransaction {
                    address,
                    signature,
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount_limit,
                });

                if mempool.push(transaction).await.is_none() {
                    return ServerError;
                };

                executor.keep_executing();
                generator.keep_generating();

                TxId(tx_id)
            }

            SellTokens {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount,
            } => {
                let tx_id = match transactions.add_new_tx().await {
                    Some(tx_id) => tx_id,
                    None => return ServerError,
                };

                let transaction = Transaction::SellTokens(SellTokensTransaction {
                    address,
                    signature,
                    base_token_id,
                    quote_token_id,
                    base_token_amount_limit,
                    quote_token_amount,
                });

                if mempool.push(transaction).await.is_none() {
                    return ServerError;
                };

                executor.keep_executing();
                generator.keep_generating();

                TxId(tx_id)
            }
        }
    })
    .await
    .unwrap();
}
