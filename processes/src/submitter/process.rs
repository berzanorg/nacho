use std::time::Duration;

use super::Processor;

pub fn process() -> Processor {
    let proof_submitter_process_script_path =
        std::env::var("NACHO_PROOF_SUBMITTER_PROCESS_SCRIPT_PATH").unwrap();

    let (stdin, stdout) = nacho_js_process::spawn(&proof_submitter_process_script_path).unwrap();

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(60 * 10)).await;

            let input = [0u8; 1];
            let mut output = [0u8; 1];

            nacho_js_process::interact(stdin, stdout, &input, &mut output)
                .await
                .ok();
        }
    });

    Processor {}
}
