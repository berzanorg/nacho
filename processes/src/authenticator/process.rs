use std::process::Stdio;

use super::{Processor, Request};
use nacho_data_structures::Transaction;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, ChildStdout, Command};
use tokio::sync::mpsc;

pub fn process(path: &str) -> Processor {
    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    let (stdin, stdout) = spawn_authenticator_script(path);

    let mut stdout = BufReader::new(stdout);

    tokio::spawn(async move {
        while let Some(request) = receiver.recv().await {
            match request {
                Request::CheckSignature { sender, tx } => {
                    let is_valid = check_signature(stdin, &mut stdout, tx).await;
                    sender.send(is_valid).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}

pub fn spawn_authenticator_script(
    path: &str,
) -> (&'static mut ChildStdin, &'static mut ChildStdout) {
    // Spawn a Node.js process
    let mut node_process = Command::new("node")
        .arg(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .unwrap();

    let stdin = node_process.stdin.take().unwrap();
    let stdout = node_process.stdout.take().unwrap();

    (Box::leak(Box::new(stdin)), Box::leak(Box::new(stdout)))
}

pub async fn check_signature(
    stdin: &mut ChildStdin,
    stdout: &mut BufReader<&mut ChildStdout>,
    tx: Transaction,
) -> Option<bool> {
    let tx_buf: [u8; 264] = (&tx).into();

    stdin.write_all(&tx_buf).await.ok()?;

    let is_valid = stdout.read_u8().await.ok()?;

    let is_valid = is_valid != 0;

    Some(is_valid)
}
