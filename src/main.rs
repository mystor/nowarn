#[macro_use]
extern crate error_chain;

use std::io::prelude::*;
use std::io::stderr;
use std::process::{exit, Command, Stdio};
use std::env;

mod error {
    error_chain!{}
}
use error::*;

quick_main!(run);
fn run() -> Result<()> {
    let mut args = env::args().skip(1);
    let prog = args.next().ok_or("Expected name of command to run")?;

    // Run the subcommand
    let output = Command::new(prog)
        .stderr(Stdio::piped())
        .args(args)
        .spawn()
        .chain_err(|| "Unable to spawn subprocess")?
        .wait_with_output()
        .chain_err(|| "Error while waiting for subprocess exit")?;

    // If the subprocess exited successfully, consume all warning output written
    // to stderr.
    if output.status.success() {
        return Ok(());
    }

    // The command exited with an error, dump out the captured stderr messages.
    stderr()
        .write_all(&output.stderr)
        .chain_err(|| "Unable to echo process's stderr messages")?;

    // Copy the command's exit code to our exit code, or exit with status `-1` if
    // the process received a signal.
    exit(output.status.code().unwrap_or(-1));
}
