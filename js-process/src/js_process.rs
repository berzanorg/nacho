use std::process::Stdio;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{ChildStdin, ChildStdout, Command},
};

use crate::error::JsProcessError;

/// Spawns a Node.js process using the given arguments and returns a the standard input and output streams of the process.
///
/// # Examples
///
/// Spawn a process:
///
/// ```rs
/// let (stdin, stdout) = nacho_js_process::spawn(&["echo.js"])?;
/// ```
///
pub fn spawn(
    args: &[&str],
) -> Result<(&'static mut ChildStdin, &'static mut ChildStdout), JsProcessError> {
    let mut process = Command::new("node")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(false)
        .spawn()?;

    let stdin = process.stdin.take().ok_or(JsProcessError::Stdin)?;
    let stdout = process.stdout.take().ok_or(JsProcessError::Stdout)?;

    Ok((Box::leak(Box::new(stdin)), Box::leak(Box::new(stdout))))
}

/// Writes the given input to the standard input stream of the process.
///
/// And reads the standart output of the process to the given output.
///
/// # Examples
///
/// Spawn a process:
///
/// ```rs
/// let (stdin, stdout) = nacho_js_process::spawn(&["greeting.js"])?;
/// ```
///
/// Interact with the process:
///
/// ```rs
/// let input = b"Berzan";
/// let mut output = [0u8; 11];
/// nacho_js_process::interact(stdin, stdout, &input, &mut output).await?;
/// assert_eq!(output, b"Hi, Berzan!");
/// ```
///
pub async fn interact(
    stdin: &mut ChildStdin,
    stdout: &mut ChildStdout,
    input: &[u8],
    output: &mut [u8],
) -> Result<(), JsProcessError> {
    stdin.write_all(input).await?;
    stdout.read_exact(output).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_echo_js_process() {
        let js_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/echo.mjs");

        let (stdin, stdout) = spawn(&[js_file_path]).unwrap();

        // First try:
        let input = [111u8; 5];
        let mut output = [0u8; 5];

        interact(stdin, stdout, &input, &mut output).await.unwrap();

        assert_eq!(output, input);

        // Second try:
        let input = [222u8; 40];
        let mut output = [0u8; 40];

        interact(stdin, stdout, &input, &mut output).await.unwrap();

        assert_eq!(output, input);
    }
}
