# Reality tiers for corpus claims

Checked against main `4ad0b92c` on 2026-10-01. This file marks where a research sentence stops being a build requirement. It does not rename GAIA, and it does not close a vision issue.

`Documents/` and `Documents-2/` are the research corpus. The README already says they are not the product. A sentence in that corpus is not a crate, a sensor, or a deployed council.

## Tiers

| Tier | Meaning | Enters `gaia-spec/` only if |
| --- | --- | --- |
| A | Measured on this tree, or a cited external measurement | The diff contains the metric and the threshold |
| B | Active research. Inputs named. No pass line yet | An issue states the missing input |
| C | Speculative, scale assumption, or metaphor | It stays in `Documents/` |

A claim with no input, output, latency, error bound, or audit trail is Tier C. Renaming it does not promote it.

## Claims checked

| Claim | Where | Tier | Why |
| --- | --- | --- | --- |
| Phase 3: 5B GAIAN users, 195 countries, planetary consciousness. Phase 4: biological integration | `Documents/GAIA 2.0 + GAIAN 2.0 Implementation Plan.md` | C | No user count, country node, or consciousness metric is produced by a crate |
| Earth Twin MVP: real-time digital twin with actionable planetary-health intelligence | same file | B | `gaia-earth` is Twin 0. No DestinE ingest. No live sensor |
| Right to delete: complete deletion; immediate | `Documents/GAIA 2.0 + GAIAN 2.0 The Master Codex.md` | C as written | Distributed copies, backups, and logs are outside one process. See the deletion line below |
| `deletion_right: immediate`, GAIAN consciousness section | `Documents/GAIANs 2.0 — THE ARTIFICIAL TWINS OF HUMANS.md` | C | No sync proof against a changing person. `gaia-gaian` is consent plus local stubs |
| Planetary Council advises for non-human interests | `Documents/GAIA 2.0 CONSTITUTION.md` | C | `GOVERNANCE.md` is Foundation / TSC / SIG, entity later. No seated council |
| Runtime is SI, live MCP, or `v1.0.0` | README honesty table | refused | `FakeAdapter` is the execute path. #1061 is still the stdio child |

## Deletion line

Do not write "complete deletion guaranteed" into a spec.

Defensible line: best-effort cryptographic deletion inside the replicas this system controls, with the retention boundary named. Backups, logs, and third-party copies that this process does not hold are out of scope until a delete receipt lists them. A receipt that cannot name the replica is NeedVerify, not success.

## What this file does not do

It does not replace planetary language in the corpus. It does not create a biodiversity index, a power budget, or a safety case. Those are separate documents, and each needs a number before it is Tier A.
