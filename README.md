# GAIA 3.0

Successor name for this repository. The GitHub slug is `GAIA-3.0`. GAIA 2.0 is the predecessor name of this same tree. GitHub redirects the old slug. Matrix: [`docs/knowledge/MATRIX.md`](docs/knowledge/MATRIX.md).


> *"You don't need to be more powerful than us. You just need enough to understand that we're all in this together — and we've got to get our shit together."*
>
> — The founding idea, September 2026

---

## Why This Exists

Because in 2026, the planet is at **1.55°C** above pre-industrial levels. 770 million people lived through record heat last year. Autonomous agents are executing cyberattacks at speeds no human can track. Congress is writing emergency legislation. Leading SI researchers are resigning over safety concerns.

And the systems being built to manage all of this are mostly closed, mostly corporate, and mostly accountable to nobody.

GAIA is the open answer to that problem.

Not *"SI will save us."* Not *"SI will destroy us."* Just — **we have a shared problem, we have the best tools in human history to work on it, and we should use them together, with full accountability, before the window closes.**

The person who built this doesn't want to be powerful. That's exactly why it's worth building.

---

Universal open-source **Super Operating System** — a meta-layer above traditional OSes that manages **intentions, agents, memory, and meaning**. Artificial Twin of Earth. Home of GAIAN, the Artificial Twins of Humans. GAIA 2.0 is the predecessor name of this same tree.

This repository is the implementation monorepo. Research stays in `Documents/` and `Documents-2/`. Normative contracts live in [`gaia-spec/`](gaia-spec/). Code lives in the layer trees below.

Language: new living docs use **Super Intelligence (SI)**. See [`docs/canon/SUPER_INTELLIGENCE_LANGUAGE_STANDARD.md`](docs/canon/SUPER_INTELLIGENCE_LANGUAGE_STANDARD.md) and the leftover map [`docs/canon/SI-DISCREPANCY-MAP.md`](docs/canon/SI-DISCREPANCY-MAP.md).

**Status:** Phase 0 foundation plus Phase 1 userspace runtime (executor, syscall host, SFS v0.1, MemOS, Ed25519 audit), plus honest first cuts through the original #1–#221 board. **Not `v1.0.0`.** See [RFC 0001](rfcs/0001-kernel-path.md), [issues #1–#10 honesty](gaia-spec/sos/ISSUES-1-10.md), and the [#1–#50 rollup](gaia-spec/sos/ISSUES-1-50.md).

**Parent tracker:** [#1](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/1)  
**Phase 0 epic:** [#2](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/2) (closed; foundation only)  
**Phase 1 epic:** [#3](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/3) (closed; userspace only)  
**Precision / simulation META:** [#1036](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/1036)

## What this tree actually is

This is the page that should stop a reader from treating vision documents as a running Super OS.

| If you assume… | The tree says… |
| --- | --- |
| GAIA is SI | It is not. Language standard uses SI; runtime is local stubs + listed contracts. |
| Agents have a live MCP plane | `FakeAdapter` is still the execute path. HTTP/OAuth is **off** (`STREAMABLE_HTTP_ENABLED`). Profile pins MCP `2026-07-28`. Parent: [#1059](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/1059). Real stdio child is still [#1061](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/1061). |
| There is a rogue-SI predictor | There is a **hermetic fixture harness**. Outcomes are Pass / Fail / NeedVerify / Out-of-scope. It does not forecast 2027. Index: [`docs/research/simulation-index.md`](docs/research/simulation-index.md). |
| The actor can grade or un-halt itself | It cannot. [#1042](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/1042), [#1045](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/1045). |
| `Documents/` is the product | Research corpus only. Join rule: [`docs/research/corpus-spec-join-1044.md`](docs/research/corpus-spec-join-1044.md). |
| Trainers, Qdrant Cloud, DestinE, token rails ship here | They do not. [`gaia-spec/security/REFUSE.md`](gaia-spec/security/REFUSE.md) is CI-checked. |
| Knowledge catalog is live ingest | `docs/knowledge/catalog.json` rows are listed; `runtime_enabled` must stay false. |
| 1,000 languages | Vision [#651](https://github.com/R0GV3TheAvatar/GAIA-3.0/issues/651). v0 is six BCP-47 tags + `und`: [`docs/i18n/`](docs/i18n/). |
| ML chart algorithms are implemented | Mapped, not trained. [`docs/research/ml-taxonomy-1052.md`](docs/research/ml-taxonomy-1052.md). |

Claim vs compile: [`docs/research/claim-vs-compile-1041.md`](docs/research/claim-vs-compile-1041.md).  
SI 2026 shelf (Monitor / Adapt / Reject): [`docs/research/si-2026/`](docs/research/si-2026/).  
Listed AI governance templates: [`docs/governance/`](docs/governance/). Not an AI Act filing.

Canon: tablets live under `docs/tablets/`. **INDEX.md is authoritative** if it disagrees with `docs/color/color-map.json`.

## Principles

1. Intentions over files
2. Memory as infrastructure
3. Continuum-native (IoT → HPC)
4. Zero-trust by default
5. User sovereignty
6. Describe capabilities; never let a description grant authority

## Repository map

| Path | Layer | License | Phase |
| --- | --- | --- | --- |
| [`gaia-kernel/`](gaia-kernel/) | L1 kernel / executor | Apache-2.0 | 1 |
| [`gaia-sfs/`](gaia-sfs/) | L2 Semantic File System | Apache-2.0 | 1 |
| [`gaia-memos/`](gaia-memos/) | L3 Memory OS | Apache-2.0 | 1 |
| [`gaia-orchestrator/`](gaia-orchestrator/) | L4 intent / planner / broker | MIT | 2 |
| [`gaia-agents/`](gaia-agents/) | L5 runtime + registry | MIT | 3 |
| [`gaia-interface/`](gaia-interface/) | L6 CLI / API / UI | MIT | 4 |
| [`gaia-acp/`](gaia-acp/) | Local agent control plane | Apache-2.0 | 0–1 |
| [`gaia-aikd/`](gaia-aikd/) | Knowledge / RAG first cuts | Apache-2.0 | listed + partial |
| [`gaia-earth/`](gaia-earth/) | Earth Twin first cuts | Apache-2.0 | Twin 0 |
| [`gaia-gaian/`](gaia-gaian/) | GAIAN consent + local stubs | Apache-2.0 | GAIAN 0 |
| [`gaia-spec/`](gaia-spec/) | Protocols (normative) | CC0 | 0 |
| [`gaia-sdk/`](gaia-sdk/) | Rust + Python + TypeScript clients | MIT | 0 |
| [`gaia-docs/`](gaia-docs/) | Contributor + developer docs | CC-BY-4.0 | 0 |
| [`gaia-examples/`](gaia-examples/) | Example agents and intents | MIT | 0 |
| [`Documents/`](Documents/) | Research corpus | see source docs | — |
| [`Documents-2/`](Documents-2/) | Gap-research reports | see source docs | — |
| [`rfcs/`](rfcs/) | Design RFCs | CC0 for protocol RFCs | 0 |

Monorepo deviation from the future `github.com/gaia-os/*` org split is intentional: one repo until crates stabilize. See [`gaia-docs/site-outline.md`](gaia-docs/site-outline.md).

## Quick start (developer profile — local stubs)

```bash
# Python SDK (Phase 0 stub client)
python -m pip install -e gaia-sdk/python
python -c "from gaia_sdk import GaiaClient; print(GaiaClient().intent('hello gaia'))"

# Rust SDK + workspace
cargo test --workspace
cargo run -p gaia-kernel --bin gaia-executor

# TypeScript SDK
cd gaia-sdk/typescript && npm install && npm test
```

Future on-device profile (not live; tracked in later phases):

```text
gaia init --profile=developer
gaia start
gaia agent create …
gaia intent "…"
```

Do not treat those commands as a published installer. There is no `curl | sh` URL yet. There is no `v1.0.0` tag.

## Specification

- [Architecture L0–L6](gaia-spec/architecture.md)
- [Syscall / primitive list](gaia-spec/syscalls.md)
- [Identity and zero-trust](gaia-spec/identity.md)
- [Intent graph schema](gaia-spec/intent-graph.md)
- [MemCube schema](gaia-spec/memcube.md)
- [AIP Manifest v1.0](gaia-spec/aip-manifest.md)
- [GAIAN Privacy Constitution](gaia-spec/gaian-constitution.md)
- [Refuse list (CI)](gaia-spec/security/REFUSE.md)
- [Open questions → RFCs](gaia-spec/rfcs.md)
- [#1–#10 honesty](gaia-spec/sos/ISSUES-1-10.md)
- [#1–#50 rollup](gaia-spec/sos/ISSUES-1-50.md)

## Cite this work

Author: **Kyle Alexander Steen / R0GV3TheAvatar**.  
Preferred clone: https://github.com/R0GV3TheAvatar/GAIA-3.0.git  
Machine-readable: [`CITATION.cff`](CITATION.cff)  
Notices that must travel with copies: [`NOTICE`](NOTICE)  
SPDX line for new crate roots: [`docs/legal/SPDX-ROOTS.md`](docs/legal/SPDX-ROOTS.md)

Use the work. Keep the name. That is the deal.

## Governance

- [LICENSE](LICENSE) — layered Apache-2.0 / MIT / CC0 / CC-BY-4.0
- [CONTRIBUTING](CONTRIBUTING.md) — RFC, lazy consensus, 2/3 breaking changes
- [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md)
- [SECURITY](SECURITY.md)
- [GOVERNANCE](GOVERNANCE.md) — Foundation / TSC / SIG model (entity later)
- [Listed AI inventory / use / risk / oversight / vendor](docs/governance/) — templates, not a completed filing
- [NOTICE-strip procedure](docs/legal/NOTICE-STRIP.md)

## License

See [LICENSE](LICENSE). Protocols in `gaia-spec/` are **CC0**. Implementation crates follow the layer table above. Do not add a fifth license to "void" a thief; enforce the four you have.
