//! Start a host process, record its id, and report why it failed.

use anyhow::{anyhow, Result};
use clap::Args;
use std::process::{Command, Stdio};
use std::time::Instant;

#[derive(Args)]
pub struct ProcArgs {
    /// Program to start. No shell.
    pub program: String,
    /// Arguments to the program.
    pub args: Vec<String>,
}

/// What one run did. Nothing here is inferred; every field was observed.
pub struct ProcReport {
    pub program: String,
    pub arg_count: usize,
    pub pid: u32,
    pub code: i32,
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
    pub millis: u128,
}

impl ProcReport {
    pub fn trace_line(&self) -> String {
        format!(
            "trace proc program={} args={} pid={} status={} ms={}",
            self.program, self.arg_count, self.pid, self.code, self.millis
        )
    }

    pub fn cause(&self) -> String {
        let first = self.stderr.lines().next().unwrap_or("no stderr");
        format!("{} exited with status {}: {}", self.program, self.code, first)
    }
}

pub fn run_process(program: &str, args: &[String]) -> Result<ProcReport> {
    let started = Instant::now();
    let child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| anyhow!("did not start {program}: {err}"))?;
    let pid = child.id();
    let output = child.wait_with_output()?;
    Ok(ProcReport {
        program: program.to_string(),
        arg_count: args.len(),
        pid,
        code: output.status.code().unwrap_or(-1),
        ok: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        millis: started.elapsed().as_millis(),
    })
}

pub async fn run(args: ProcArgs) -> Result<()> {
    let report = run_process(&args.program, &args.args)?;
    println!(
        "pid={} status={} program={}",
        report.pid, report.code, report.program
    );
    print!("{}", report.stdout);
    println!("{}", report.trace_line());
    if !report.ok {
        eprint!("{}", report.stderr);
        return Err(anyhow!(report.cause()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn this_binary() -> String {
        std::env::current_exe().unwrap().display().to_string()
    }

    #[test]
    fn proc_failure_reports_its_cause() {
        let report = run_process(&this_binary(), &["--no-such-flag".to_string()]).unwrap();
        assert!(!report.ok);
        assert!(!report.stderr.is_empty(), "stderr was dropped");
        assert!(report.cause().contains("exited with status"));
    }

    #[test]
    fn proc_run_is_traced() {
        let report = run_process(&this_binary(), &["--list".to_string()]).unwrap();
        assert!(report.ok);
        let line = report.trace_line();
        assert!(line.starts_with("trace proc program="));
        assert!(line.contains(&format!("pid={}", report.pid)));
    }

    #[test]
    fn proc_missing_program_misses_loud() {
        let result = run_process("gaia-no-such-program", &[]);
        assert!(result.is_err());
    }
}
