use std::time::Duration;

use super::Processor;
use crate::transactions;
use tokio::{
    process::{ChildStdin, ChildStdout},
    sync::mpsc,
    time::sleep,
};

pub fn process(path: &str, transactions: transactions::Processor) -> Processor {
    let (sender, mut receiver) = mpsc::channel::<u32>(1000);

    let (mut stdin, mut stdout) = nacho_js_process::spawn(&[path]).unwrap();

    tokio::spawn(async move {
        let sleep = sleep(Duration::from_millis(100));
        tokio::pin!(sleep);
        loop {
            tokio::select! {
                _ = &mut sleep => {
                    let maybe_merged_until = transactions.get_merged_until().await;
                    let maybe_proved_until = transactions.get_proved_until().await;

                    if let Some((merged_until, proved_until)) = maybe_merged_until.zip(maybe_proved_until) {
                        if proved_until <= 1 {
                            continue;
                        }

                        if merged_until == 0 {
                            if start_merge(&mut stdin, &mut stdout, 0).await.is_ok() {
                                transactions.set_merged_until(2).await;
                            }
                            continue;
                        }

                        if proved_until > merged_until {
                            if continue_merge(&mut stdin, &mut stdout, merged_until as u32).await.is_ok() {
                                transactions.set_merged_until(merged_until + 1).await;
                            }
                            continue;
                        }
                    }

                }
                msg = receiver.recv() => {
                    if let Some(at) = msg {
                        if start_merge(&mut stdin, &mut stdout, at).await.is_ok() {
                            transactions.set_merged_until(at as u64 + 2).await;
                        }
                    }
                },

            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}

pub async fn start_merge(
    stdin: &mut ChildStdin,
    stdout: &mut ChildStdout,
    at: u32,
) -> Result<(), ()> {
    let mut input = [0u8; 5];
    let mut output = [0u8; 1];

    input[0] = 0;
    input[1..5].copy_from_slice(&at.to_le_bytes());

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

pub async fn continue_merge(
    stdin: &mut ChildStdin,
    stdout: &mut ChildStdout,
    at: u32,
) -> Result<(), ()> {
    let mut input = [0u8; 5];
    let mut output = [0u8; 1];

    input[0] = 1;
    input[1..5].copy_from_slice(&at.to_le_bytes());

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
