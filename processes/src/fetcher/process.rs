use std::time::Duration;

use super::Processor;
use crate::{
    balances, burns, executor, liquidities, mempool, pools, proofpool, transactions, verifier,
    withdrawals,
};
use nacho_data_structures::{
    ByteConversion, Deposit, DepositTokensTransaction, Transaction, Withdrawal,
};
use nacho_events_db::EventsDb;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{ChildStdin, ChildStdout},
};

pub fn process(
    events_db_path: &str,
    event_fetcher_process_path: &str,
    verifier: verifier::Processor,
    executor: executor::Processor,
    mempool: mempool::Processor,
    transactions: transactions::Processor,
    proofpool: proofpool::Processor,
    balances: balances::Processor,
    pools: pools::Processor,
    liquidities: liquidities::Processor,
    burns: burns::Processor,
    withdrawals: withdrawals::Processor,
) -> Processor {
    let (stdin, stdout) = nacho_js_process::spawn(&[event_fetcher_process_path]).unwrap();

    let events_db_path = events_db_path.to_string();

    tokio::spawn(async move {
        let mut events_db = EventsDb::new(events_db_path).await.unwrap();

        loop {
            tokio::time::sleep(Duration::from_secs(60 * 1)).await;

            let (mut from_block_deposited, mut from_block_withdrawn) =
                match events_db.get_last_fetched_blocks().await {
                    Ok(value) => value,
                    Err(_) => continue,
                };

            match fetch_deposited_events(stdin, stdout, from_block_deposited).await {
                Some((last_fetched_block, deposited_events)) => {
                    from_block_deposited = last_fetched_block;

                    let deposit_transactions = deposited_events.into_iter().map(|event| {
                        Transaction::DepositTokens(DepositTokensTransaction {
                            user_address: event.depositor,
                            token_id: event.token_id,
                            token_amount: event.token_amount,
                        })
                    });

                    for transaction in deposit_transactions {
                        if let Some(_) = transactions.add_new_tx().await {
                            mempool.push(transaction).await;
                        }
                    }

                    executor.keep_executing();
                }
                None => (),
            };

            match fetch_withdrawn_events(stdin, stdout, from_block_withdrawn).await {
                Some((last_fetched_block, withdrawn_events)) => {
                    from_block_withdrawn = last_fetched_block;

                    for event in withdrawn_events {
                        if let Some(index) = burns
                            .get_index(event.withdrawer.clone(), event.token_id.clone())
                            .await
                        {
                            withdrawals.set(index, event).await;
                        }
                    }
                }
                None => (),
            };

            events_db
                .set_last_fetched_blocks(from_block_deposited, from_block_withdrawn)
                .await
                .ok();
        }
    });

    Processor {}
}

pub async fn fetch_deposited_events(
    stdin: &mut ChildStdin,
    stdout: &mut ChildStdout,
    from_block: u32,
) -> Option<(u32, Vec<Deposit>)> {
    let mut input = [0u8; 5];

    input[0] = 0;
    input[1..5].copy_from_slice(&from_block.to_bytes());

    let mut output = [0u8; 4];

    stdin.write_all(&input).await.ok()?;

    stdout.read_exact(&mut output).await.ok()?;
    let last_block_fetched = match u32::from_bytes(&output) {
        0 => from_block,
        x => x,
    };

    stdout.read_exact(&mut output).await.ok()?;
    let events_count = u32::from_bytes(&output);

    let mut events = Vec::with_capacity(events_count as usize);

    for _ in 0..events_count {
        let mut output = [0u8; 95];
        stdout.read_exact(&mut output).await.ok()?;
        let event = Deposit::from_bytes(&output);
        events.push(event);
    }

    Some((last_block_fetched, events))
}

pub async fn fetch_withdrawn_events(
    stdin: &mut ChildStdin,
    stdout: &mut ChildStdout,
    from_block: u32,
) -> Option<(u32, Vec<Withdrawal>)> {
    let mut input = [0u8; 5];

    input[0] = 1;
    input[1..5].copy_from_slice(&from_block.to_bytes());

    let mut output = [0u8; 4];

    stdin.write_all(&input).await.ok()?;

    stdout.read_exact(&mut output).await.ok()?;
    let last_block_fetched = match u32::from_bytes(&output) {
        0 => from_block,
        x => x,
    };

    stdout.read_exact(&mut output).await.ok()?;
    let events_count = u32::from_bytes(&output);

    let mut events = Vec::with_capacity(events_count as usize);

    for _ in 0..events_count {
        let mut output = [0u8; 95];
        stdout.read_exact(&mut output).await.ok()?;
        let event = Withdrawal::from_bytes(&output);
        events.push(event);
    }

    Some((last_block_fetched, events))
}
