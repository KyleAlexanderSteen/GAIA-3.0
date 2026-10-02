# Stub sweep — #1304

Date: 2026-10-02. Scope: `gaia-cli/src` and `gaia-gateway/src` after the local echo cut.

| Path | What it does now | Success claim? |
| --- | --- | --- |
| `gaia-cli` `init --profile developer` | Writes `$GAIA_HOME/profile.toml` with `runtime = not-started`. Does not connect. | No runtime claim |\n| `gaia-cli` `init --profile` other than developer, `start`, `agent`, `memory`, `revoke` | `not_implemented`, exit non-zero | No |
| `gaia-cli` `intent` without gateway | Executor receipt. `echo:` records to `$GAIA_HOME/receipts.jsonl` (default `.gaia`). Plain text is `not-executed`. | Only after record |
| `gaia-cli` `intent --stream` / `--gateway` | not implemented | No |
| `gaia-cli` `audit` | Reads the local ledger, or prints `no local receipts` | Empty ledger is an empty read, not a completed remote audit |
| `gaia-cli` `audit --follow` | not implemented | No |
| `gaia-gateway` `POST /intent` | Same executor rule. `echo:` is 200 `recorded`. Other text is 501. | No `queued` |
| `gaia-gateway` `GET /intent/stream` | 501 | No stub SSE event |
| `gaia-gateway` agent create/deploy | 501 | No `created` / `deploying` |
| `gaia-gateway` memory list/search | 501 | No empty cube list |
| `gaia-gateway` audit get/stream | 501 | No empty entry list |
| `gaia-gateway` `GET /health` | 204, process is up | Real |
| `gaia-gateway` revoke | 204 if this process holds the handle, else 404 | Real |
| `gaia-kernel` `invoke` / `observe` | `KernelError::NotImplemented` | No |
| `gaia-kernel` `run_intent` | `recorded` / `refused` / `not-executed`. Never `done`. | `done` still means a syscall |

CI: `python scripts/check_stub_success.py` fails if a TODO in these two trees sits next to `queued`, `Runtime started`, `"created"`, `"deploying"`, or `status: "done"`.


## Sweep outside CLI and gateway (2026-10-02)

| Path | Result |
| --- | --- |
| `gaia-sdk` Rust `intent`, `context`, `invoke`, `observe`, `declare` | `GaiaError::NotImplemented`. Sign and verify still perform Ed25519. |
| `gaia-sdk` Python same methods | `NotImplementedCapability`. Sign and verify already raised. |
| `gaia-orchestrator` `dispatch_tool` | Returns `mcp-not-executed:` with `provenance=local-stub`. Not `mcp-ok`. |
| `gaia-orchestrator` `gaia intent` without `--accept` | Prints `execution=not-started`. |
| `gaia-kernel` `invoke` / `observe` | `KernelError::NotImplemented` (landed in #1351). |
| `gaia-agents` pack descriptions | Labeled stub text. `act()` still refuses. |
| Metrics `TODO(#734)` | Comment only. No success counter is emitted. |

CI scans every `gaia-*/src` for a TODO next to `queued`, `Runtime started`, `"created"`, `"deploying"`, `status: "done"`, `mcp-ok:`, or `admitted`.
