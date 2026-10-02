# Stub sweep — #1304

Date: 2026-10-02. Scope: `gaia-cli/src` and `gaia-gateway/src` after the local echo cut.

| Path | What it does now | Success claim? |
| --- | --- | --- |
| `gaia-cli` `init`, `start`, `agent`, `memory`, `revoke` | `not_implemented`, exit non-zero | No |
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
