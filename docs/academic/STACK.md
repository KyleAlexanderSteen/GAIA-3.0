# GAIA 2.0 Academic Stack

**Proof:** PROOF-GAIA20-STACK-001 (empirical, partial)
**Date:** 2026-09-17

## Live systems

| Plane | Resource | Status |
|---|---|---|
| GitHub | https://github.com/KyleAlexanderSteen/GAIA-2.0 | public, this commit |
| Supabase | org `GAIA 2.0`, project `gaia-2-0`, ref `yylqoiqobydrdsnnulip`, region `us-east-2` | ACTIVE_HEALTHY |
| Hugging Face | https://huggingface.co/KyleAlexanderSteen | authenticated; dataset card staged here until Hub create is available |

## Supabase tables (RLS on)

- `memories` — CT-002 HOT/WARM/COLD + relevance + Metric 6 view `memory_retention_metric`
- `consent_events` — CT-003/CT-004 ledger + shard_key
- `agent_health` — CT-003 circuit breaker + Safety/Consent hot-standby
- `elements` / `minerals` / `crystals` — lithic stack
- `proofs` — THE ORDER registry

## Seed (2026-09-17)

- 10 agent_health rows (Safety/Consent primary+standby)
- elements O, Si, Fe
- mineral + crystal `quartz`
- proof `PROOF-GAIA20-STACK-001`
