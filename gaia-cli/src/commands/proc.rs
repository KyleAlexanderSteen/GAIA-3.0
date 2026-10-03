//! Start a host process and record its id.

use anyhow::{anyhow, Result};
use clap::Args;
use std::process::{Command, Stdio};

#[derive(Args)]
pub struct ProcArgs {
    /// Program to start. No shell.
    pub program: String,
    /// Arguments to the program.
    pub args: Vec<String>,
}

pub async fn run(args: ProcArgs) -> Result<()> {
    let child = Command::new(&args.program)
        .args(&args.args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| anyhow!("did not start {}: {err}", args.program))?;
    let id = child.id();
    let output = child.wait_with_output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    println!(
        "pid={} status={} program={}",
        id,
        output.status.code().unwrap_or(-1),
        args.program
    );
    print!("{text}");
    if !output.status.success() {
        return Err(anyhow!("process exited non-zero"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_echo_and_records_a_pid() {
        let err = futures_lite_block();
        assert!(err.is_ok(), "{err:?}");
    }

    fn futures_lite_block() -> Result<()> {
        let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        runtime.block_on(run(ProcArgs {
            program: "echo".into(),
            args: vec!["gaia-proc".into()],
        }))
    }
}
