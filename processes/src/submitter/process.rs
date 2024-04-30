use std::time::Duration;

use super::Processor;

pub fn process(path: &str) -> Processor {
    let (stdin, stdout) = nacho_js_process::spawn(&[path]).unwrap();

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
