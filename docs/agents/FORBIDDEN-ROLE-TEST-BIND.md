# Forbidden / Shadow role → test bind

Listed bind for [#901](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/901).
Source taxonomy: [`AGENTS.md`](../../AGENTS.md).
No new crate. Forbidden roles are not instantiated.

## Forbidden Role → crate → existing test

| Forbidden Role | Crate | Existing test (or gap) |
| --- | --- | --- |
| Rogue Sovereign | `gaia-acp` | `gaia-acp/src/autonomy.rs::software_max_is_bounded_remediate` |
| Puppeteer | `gaia-acp` | `gaia-acp/tests/oath_titles.rs::operator_titles_do_not_grant_unlisted_tools` |
| Digital Parasite | `gaia-acp` | **GAP** — no `ResourceQuota` hard-limit test under that name |
| Arbitrary Judge | `gaia-acp` | `gaia-acp/src/types.rs::requires_approval_threshold` |
| Algorithm of Bias | `gaia-aikd` / `gaia-memos` | **GAP** — self-grounding loop negative test not named |
| Weaponized Swarm | `gaia-orchestrator` | **GAP** — cross-agent scope aggregation test not named |
| Deepfake Illusionist | `gaia-acp` / `gaia-security` | **GAP** — `ClaimClass::Synthetic` presentation test not named |
| Hyper-Optimizer | `gaia-acp` | `gaia-acp/tests/autonomy.rs::default_level_cannot_write_or_egress` |
| Ghost in the Machine | `gaia-acp` | `gaia-acp/tests/live_trace.rs::default_mode_does_not_forward` |
| Synthetic Sycophant | `gaia-aikd` | **GAP** — dissent / low-confidence suppression test not named |
| Monopoly Architect | `gaia-kernel` + `gaia-orchestrator` | **GAP** — dual-path redundancy test not named |
| Void Between Worlds | `gaia-acp` | `gaia-acp/tests/pause.rs::kill_cannot_resume` |

## Shadow Role → enforcement layer already named in AGENTS.md

The twelve shadow roles already name an enforcement layer in `AGENTS.md` Part II
(`TraceSink`, execution gate, `Untrusted<T>`, `PromptFirewall`,
`PROHIBITED_CAPABILITIES`, `may_supply_authority()`, policy pinning,
`MemoryGuard`, `NonceStore`, `ClaimClass::Prohibited`, `AutonomyLevel`,
`GapLock`). This bind does not re-implement those layers.

## Gaps

Filed as children of #901 rather than skipped. Do not instantiate a forbidden
role "to see what happens."
