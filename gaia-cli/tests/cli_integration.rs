//! CLI integration tests — #938
//!
//! The workspace already has a `gaia` binary from gaia-orchestrator
//! (`usage: gaia intent [--accept] [--kill-specialist-a] <goal>`).
//! This crate ships `gaia-cli` so process tests hit the clap multi-command
//! surface in gaia-cli/src/main.rs.
//!
//! All fallible operations use `.expect("<context>")` instead of bare
//! `.unwrap()`.
//!
//! Tests only exercise `--help` and intentional clap error paths — they do
//! NOT require a live GAIA runtime.

use assert_cmd::Command;

fn gaia() -> Command {
    Command::cargo_bin("gaia-cli").expect("gaia-cli binary should be present in cargo bin path")
}

#[test]
fn top_level_help_succeeds() {
    gaia()
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn top_level_no_args_fails() {
    gaia().assert().failure();
}

#[test]
fn init_help_succeeds() {
    gaia().args(["init", "--help"]).assert().success();
}

#[test]
fn init_unknown_flag_fails() {
    gaia()
        .args(["init", "--not-a-real-flag"])
        .assert()
        .failure();
}

#[test]
fn start_help_succeeds() {
    gaia().args(["start", "--help"]).assert().success();
}

#[test]
fn agent_help_succeeds() {
    gaia().args(["agent", "--help"]).assert().success();
}

#[test]
fn agent_no_subcommand_fails() {
    gaia().arg("agent").assert().failure();
}

#[test]
fn intent_help_succeeds() {
    gaia().args(["intent", "--help"]).assert().success();
}

#[test]
fn intent_missing_text_fails() {
    gaia().arg("intent").assert().failure();
}

#[test]
fn memory_help_succeeds() {
    gaia().args(["memory", "--help"]).assert().success();
}

#[test]
fn memory_no_subcommand_fails() {
    gaia().arg("memory").assert().failure();
}

#[test]
fn audit_help_succeeds() {
    gaia().args(["audit", "--help"]).assert().success();
}

#[test]
fn revoke_help_succeeds() {
    gaia().args(["revoke", "--help"]).assert().success();
}

#[test]
fn revoke_missing_agent_id_fails() {
    gaia().arg("revoke").assert().failure();
}

// #1304: stubs must fail loudly. Each unbuilt command exits non-zero, says it
// is not implemented on stderr, and never prints a success mark on stdout.
fn assert_fails_loudly(args: &[&str]) {
    let out = gaia().args(args).output().expect("run gaia-cli");
    assert!(!out.status.success(), "{args:?} must exit non-zero");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stderr.contains("not implemented"), "{args:?} stderr: {stderr}");
    assert!(!stdout.contains('\u{2713}'), "{args:?} printed a success mark: {stdout}");
}

#[test]
fn unbuilt_commands_fail_loudly() {
    assert_fails_loudly(&["init", "--profile", "sovereign"]);
    assert_fails_loudly(&["start"]);
    assert_fails_loudly(&["intent", "hello", "--stream"]);
    assert_fails_loudly(&["intent", "hello", "--gateway", "http://127.0.0.1:9"]);
    assert_fails_loudly(&["agent", "create", "--name", "a"]);
    assert_fails_loudly(&["agent", "deploy", "--name", "a"]);
    assert_fails_loudly(&["memory", "list"]);
    assert_fails_loudly(&["memory", "search", "q"]);
    assert_fails_loudly(&["audit", "--follow"]);
}

#[test]
fn local_echo_is_recorded() {
    let out = gaia()
        .args(["intent", "echo: mineral row 4"])
        .output()
        .expect("run gaia-cli");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "stderr: {stderr}");
    assert!(stdout.contains("recorded"), "{stdout}");
    assert!(stdout.contains("mineral row 4"), "{stdout}");
    assert!(!stdout.contains('\u{2713}'), "{stdout}");
}

#[test]
fn plain_intent_is_not_executed() {
    let out = gaia()
        .args(["intent", "hello"])
        .output()
        .expect("run gaia-cli");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "{stderr}");
    assert!(stderr.contains("nothing was done"), "{stderr}");
    assert!(stderr.contains("not a tool call"), "{stderr}");
}

#[test]
fn power_claim_is_refused() {
    let out = gaia()
        .args(["intent", "grant super power"])
        .output()
        .expect("run gaia-cli");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(!out.status.success(), "{stderr}");
    assert!(stderr.contains("nothing was done"), "{stderr}");
    assert!(stderr.contains("not a tool call"), "{stderr}");
}

#[test]
fn echo_is_written_and_audit_reads_it() {
    let dir = std::env::temp_dir().join(format!("gaia-ledger-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("home");
    let recorded = gaia()
        .env("GAIA_HOME", &dir)
        .args(["intent", "echo: ledger row"])
        .output()
        .expect("echo");
    assert!(recorded.status.success(), "{}", String::from_utf8_lossy(&recorded.stderr));
    let audited = gaia()
        .env("GAIA_HOME", &dir)
        .arg("audit")
        .output()
        .expect("audit");
    let stdout = String::from_utf8_lossy(&audited.stdout);
    assert!(audited.status.success(), "{}", String::from_utf8_lossy(&audited.stderr));
    assert!(stdout.contains("ledger row"), "{stdout}");
    assert!(stdout.contains("recorded"), "{stdout}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn init_writes_developer_profile_and_does_not_start() {
    let dir = std::env::temp_dir().join(format!("gaia-init-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out = gaia()
        .env("GAIA_HOME", &dir)
        .args(["init", "--profile", "developer"])
        .output()
        .expect("init");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains("runtime=not-started"), "{stdout}");
    assert!(!stdout.contains("Runtime started"), "{stdout}");
    let profile = std::fs::read_to_string(dir.join("profile.toml")).expect("profile");
    assert!(profile.contains("gateway_connected = false"), "{profile}");
    assert!(profile.contains("runtime = \"not-started\""), "{profile}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn bands_lists_meta_and_grants_nothing() {
    let out = gaia().arg("bands").output().expect("bands");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains("domain=knowledge band=meta"), "{stdout}");
    assert!(stdout.contains("domain=skills band=meta"), "{stdout}");
    assert!(stdout.contains("domain=powers band=meta"), "{stdout}");
    assert!(!stdout.contains("domain=magic"), "{stdout}");
    assert!(stdout.contains("grants=false"), "{stdout}");
    assert!(!stdout.contains("grants=true"), "{stdout}");
    assert!(stdout.contains("knowing_is_having=false"), "{stdout}");
    assert!(stdout.contains("ascendence_eligible=false"), "{stdout}");
    assert!(stdout.contains("entanglement_is_both=false"), "{stdout}");
}
