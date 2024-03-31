use nacho_rpc_server::{start_rpc_server, RpcMethod::*, RpcResponse::*};

#[tokio::main]
async fn main() {
    let balances = nacho_processes::balances::process("/tmp/nacho/balances");
    let burns = nacho_processes::burns::process("/tmp/nacho/burns");
    let liquidities = nacho_processes::liquidities::process("/tmp/nacho/liquidities");
    let mempool = nacho_processes::mempool::process("/tmp/nacho/mempool");
    let pools = nacho_processes::pools::process("/tmp/nacho/pools");
    let proofpool = nacho_processes::proofpool::process("/tmp/nacho/proofpool");
    let transactions = nacho_processes::transactions::process("/tmp/nacho/transactions");
    let withdrawals = nacho_processes::withdrawals::process("/tmp/nacho/withdrawals");
    let executor =
        nacho_processes::executor::process(mempool, proofpool, balances, pools, liquidities, burns);

    start_rpc_server("127.0.0.1:2345", move |method| async move {
        match method {
            Unknown => todo!(),

            GetTotalTxCount => {
                let total_tx_count = transactions.get_tx_count().await;

                match total_tx_count {
                    Some(total_tx_count) => TotalTxCount(total_tx_count),
                    None => ServerError,
                }
            }

            GetTxStatus { tx_id } => todo!(),

            GetBalances { address } => todo!(),

            GetPools => todo!(),

            GetLiquidities { address } => todo!(),

            GetBurns { address } => todo!(),

            GetBridgeWitnesses { burn_id } => todo!(),

            BurnTokens {
                address,
                signature,
                token_id,
                token_amount,
            } => todo!(),

            CreatePool {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount,
            } => todo!(),

            ProvideLiquidity {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount_limit,
            } => todo!(),

            RemoveLiquidity {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount_limit,
                liquidity_point_amount,
            } => todo!(),

            BuyTokens {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount_limit,
            } => todo!(),

            SellTokens {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount,
            } => todo!(),
        }
    })
    .await
    .unwrap();
}
