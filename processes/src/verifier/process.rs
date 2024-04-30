use super::{Processor, Request};
use nacho_data_structures::ByteConversion;
use tokio::sync::mpsc;

pub fn process() -> Processor {
    let signature_verifier_process_script_path =
        std::env::var("NACHO_SIGNATURE_VERIFIER_PROCESS_SCRIPT_PATH").unwrap();

    let (sender, mut receiver) = mpsc::channel::<Request>(1000);

    let (stdin, stdout) = nacho_js_process::spawn(&signature_verifier_process_script_path).unwrap();

    tokio::spawn(async move {
        while let Some(request) = receiver.recv().await {
            match request {
                Request::CheckSignature { sender, tx } => {
                    let input = tx.to_bytes();
                    let mut output = [0u8; 1];

                    nacho_js_process::interact(stdin, stdout, &input, &mut output)
                        .await
                        .ok();

                    let is_valid = output[0] != 0;

                    sender.send(Some(is_valid)).unwrap();
                }
            }
        }
    });

    Processor {
        sender: Box::leak(Box::new(sender)),
    }
}
