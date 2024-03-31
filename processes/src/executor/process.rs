use super::{Processor, Request};
use crate::{balances, burns, liquidities, mempool, pools, proofpool};
use nacho_data_structures::Transaction;
use tokio::sync::mpsc;

pub fn process(
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
                    let tx = match mempool.pop().await {
                        Some(tx) => {
                            let result = execute_tx(
                                tx,
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
                        None => {
                            sender.send(None).unwrap();
                            continue;
                        }
                    };
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}

pub async fn execute_tx(
    tx: Transaction,
    mempool: mempool::Processor,
    proofpool: proofpool::Processor,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
) -> Result<(), ()> {
    match tx {
        Transaction::BurnToken {
            address,
            signature,
            token_id,
            token_amount,
        } => {}
        Transaction::CreatePool {
            address,
            signature,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount,
        } => todo!(),
        Transaction::ProvideLiquidity {
            address,
            signature,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount_limit,
        } => todo!(),
        Transaction::RemoveLiquidity {
            address,
            signature,
            base_token_id,
            quote_token_id,
            base_token_amount_limit,
            quote_token_amount_limit,
            liquidity_point_amount,
        } => todo!(),
        Transaction::BuyTokens {
            address,
            signature,
            base_token_id,
            quote_token_id,
            base_token_amount,
            quote_token_amount_limit,
        } => todo!(),
        Transaction::SellTokens {
            address,
            signature,
            base_token_id,
            quote_token_id,
            base_token_amount_limit,
            quote_token_amount,
        } => todo!(),
    }

    Ok(())
}
