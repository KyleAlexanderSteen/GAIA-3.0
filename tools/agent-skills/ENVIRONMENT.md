# Agent environment requirements

What an AI agent needs in order to build, test and ship changes to this repository. Checked 2026-10-01 against the agent sandbox used for PRs #1323 and #1324.

## Current state

| Capability | Status |
|---|---|
| git, python3, node 20, npm, jq, curl | Available. |
| GitHub connection (read files, branches, PRs, comments, reviews) | Works. Used to open #1323 and #1324. |
| `gh` command line | Installed, not logged in. |
| `git push` from the sandbox | No credentials. Small files are pushed through the GitHub connection instead. |
| Rust (`cargo`, `rustc`, `rustfmt`, `clippy`) | **Missing.** |
| Docker | Missing. |
| Persistent working copy | None. The sandbox does not keep files between turns; the repo must be re-cloned and anything worth keeping must be on GitHub. |
| Disk | About 1.3 GB free. A full workspace build may not fit. |
| Command time limit | 5 minutes. |

## Needed, in priority order

1. **Rust toolchain.** Without it an agent cannot run `cargo test` or `cargo clippy --all-targets`, so it cannot safely change any `gaia-*` crate. Install with `rustup`; if disk is short, build one crate at a time (`cargo test -p gaia-skills`). Unverified until tried.
2. **A way to push.** Either a fine-grained token scoped to this repository only (contents, pull requests and issues write; nothing else), given to the sandbox and never committed, then revoked when done; or a human runs bulk scripts such as `rename-to-gaia-3.0.sh` locally.
3. **CI that runs the checks automatically.** The tools in this directory do nothing until a workflow runs them: unit tests on every change, `dod_check.py` on every PR body, and `stub_detector.py` on the Rust sources. Not done yet.
4. **Long-running builds off the sandbox.** Full Rust builds and long test runs exceed the time limit. GitHub Actions can run them; the agent pushes a branch and reads the results.

## What works without any of the above

- Writing and testing Python checks, docs, templates and workflow files.
- Reading the whole repository, reviewing PRs, opening and commenting on issues.
- Anything that finishes in under 5 minutes and fits in the disk space.

## Not yet verified

- Whether `rustup` installs and builds within the disk limit.
- Whether a workflow can pass the PR body to `dod_check.py` (it can read it from `github.event.pull_request.body`, but this has not been run).
