pub mod init;
pub mod start;
pub mod agent;
pub mod intent;
pub mod memory;
pub mod audit;
pub mod revoke;

/// Rule (#1304): a command whose real behavior is not built must fail loudly.
/// It prints nothing to stdout, writes a clear message to stderr, and exits
/// non-zero. It must never print a success line.
pub fn not_implemented(command: &str, issue: &str) -> anyhow::Error {
    anyhow::anyhow!("`gaia {command}` is not implemented yet (tracked in {issue}); nothing was done")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_implemented_names_command_and_issue_and_says_nothing_was_done() {
        let msg = not_implemented("start", "#1298").to_string();
        assert!(msg.contains("gaia start"));
        assert!(msg.contains("#1298"));
        assert!(msg.contains("nothing was done"));
        assert!(!msg.contains('\u{2713}'), "must not contain a success mark");
    }
}
