# GAIA Preflight

GAIA Preflight is a read-only gate for catching integration failures before a pull request is opened or updated.

## Checks

- Working-tree cleanliness.
- Synchronization with the selected base branch (default: `main`).
- Ahead/behind and divergence state.
- Known merge hotspots: `gaia-kernel/src/lib.rs`, `gaia-cli/src/main.rs`, and `gaia-cli/src/commands/mod.rs`.
- Rust module declarations whose source files are missing.
- `rustfmt` for changed Rust files.
- Workspace `clippy` with warnings denied.
- Workspace tests, unless `--skip-tests` is supplied.

## Usage

```bash
scripts/gaia-preflight.sh
scripts/gaia-preflight.sh --base main
scripts/gaia-preflight.sh --skip-tests
```

The script does not rebase, merge, edit files, push branches, change PRs, or alter CI configuration.

## Result semantics

- **PASS** — condition verified.
- **WARN** — review-worthy condition that is not automatically blocking.
- **BLOCK** — resolve the condition before opening or updating the PR.
- **READY** — no blocking checks.
- **READY WITH WARNINGS** — no blocking checks, but warnings exist.
- **BLOCKED** — one or more blocking checks failed.

## Relationship to the correction loop

The existing `docs/agent-correction-loop.md` is the post-PR human-gated repair protocol. Preflight complements it by catching branch synchronization, registry contention, missing modules, formatting, Clippy, and test failures before another PR enters the correction loop.
