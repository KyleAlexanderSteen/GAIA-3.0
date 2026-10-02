//! `gaia-validate` — MCP service layer for the GAIA-2.0 human-gated
//! correction loop.
//!
//! # Overview
//!
//! This crate exposes three public operations used by AI contributors
//! during a correction loop turn:
//!
//! * [`validate`] — run `scripts/agent-validate.sh` and return a typed result.
//! * [`get_validation_result`] — deserialise an existing `agent-validation.json`.
//! * [`get_attempt_history`] — load and sort all attempt files from a directory.
//!
//! All operations are synchronous and have no network dependency.

pub mod error;
pub mod ontology_gate;
pub mod ontology_boot;
pub mod result;
pub mod history;
pub mod runner;

pub use error::ValidateError;
pub use ontology_gate::{admit_consciousness, admit_magic, admit_power, GateError, MagicClaim, PowerClaim, PowerState};
pub use ontology_boot::{boot_line, OntologyBoot};
pub use result::ValidationResult;
pub use history::AttemptHistory;

use std::path::Path;

/// Run `scripts/agent-validate.sh` and return a [`ValidationResult`].
///
/// # Arguments
///
/// * `mode`              — `"full"`, `"changed"`, or `"targeted"`.
/// * `attempt`           — 1-indexed correction loop attempt number.
/// * `prior_result_path` — Optional path to the previous
///   `agent-validation.json`.  When supplied the script receives
///   `--fingerprint <path>` so it can detect no-progress.
///
/// # Errors
///
/// Returns [`ValidateError`] if the script is not found, fails to launch,
/// produces invalid JSON output, or exits with code 2 (usage error).
/// Exit code 1 (failures) and exit code 3 (no progress) are **not** errors
/// — they are represented as fields inside the returned [`ValidationResult`].
pub fn validate(
    mode: &str,
    attempt: u32,
    prior_result_path: Option<&Path>,
) -> Result<ValidationResult, ValidateError> {
    runner::run(mode, attempt, prior_result_path)
}

/// Deserialise an existing `agent-validation.json` without re-running checks.
///
/// Use this to read a result produced by a previous loop attempt, a CI
/// artifact, or any call to [`validate`] whose output was already written to
/// disk.
///
/// # Errors
///
/// Returns [`ValidateError::Io`] if the file cannot be read, or
/// [`ValidateError::Parse`] if the JSON is malformed.
pub fn get_validation_result(path: &Path) -> Result<ValidationResult, ValidateError> {
    let raw = std::fs::read_to_string(path)
        .map_err(|e| ValidateError::Io { path: path.to_path_buf(), source: e })?;
    serde_json::from_str(&raw)
        .map_err(|e| ValidateError::Parse { path: path.to_path_buf(), source: e })
}

/// Load and sort all attempt result files from `dir`.
///
/// The function scans `dir` for files named `agent-validation-N.json`
/// (N ≥ 1) and `agent-validation.json` (treated as attempt 1 when no
/// numbered files exist), parses each one, and returns an [`AttemptHistory`]
/// whose entries are sorted ascending by attempt number.
///
/// Files that cannot be parsed are silently skipped so that a single
/// corrupt artifact does not break the whole history.
///
/// # Errors
///
/// Returns [`ValidateError::Io`] only if `dir` itself cannot be read.
pub fn get_attempt_history(dir: &Path) -> Result<AttemptHistory, ValidateError> {
    history::load(dir)
}

/// Returns `true` if `current` was produced against `expected_sha`.
///
/// The correction loop **must** call this before proposing any repair.
/// A stale result (wrong `head_sha`) must be rejected — the loop should
/// re-run validation against the actual current HEAD.
pub fn is_sha_current(result: &ValidationResult, expected_sha: &str) -> bool {
    result.head_sha == expected_sha
}
