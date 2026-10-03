# Trace boundary — STRIDE, SRE, Saela mapping

Status: documentation only. Distinguishes **schema deployed** from **runtime verified**.

Related: [Issue #335](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/335), [Issue #334](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/334), `gaia-orchestrator/docs/TRACE_EVENTS.md`.

## Claim boundary

| Layer | State on 2026-09-19 |
| --- | --- |
| Supabase tables `trace_events`, `gap_locks`, control-plane functions | Schema deployed in project `gaia-2-0` |
| Local `TraceEventSink` + `permit_execution` | Code verified in-repo; no network |
| `LocalRunner::run_with_gate` | Wired and unit-tested against in-memory gate |
| `LiveTraceRow` column names | Mapped to deployed schema; still no HTTP |
| Runtime writes to Supabase | **Not implemented, not claimed** |
| Hugging Face Hub | Not a control plane or audit store |

## STRIDE (local seam only)

| Threat | Treatment now | Not claimed |
| --- | --- | --- |
| Spoofing | Kernel Ed25519 audit remains authoritative; telemetry has no authz power | Remote principal proof via `trace_events` |
| Tampering | Hash-chained `AuditLog` first; sink cannot rewrite the chain | Integrity of a future remote row |
| Repudiation | Local sequence + chain; reason codes are allow-listed | Legal-grade remote evidence |
| Information disclosure | No raw prompts/secrets in `TraceEvent`; unknown text → `GAIA_OTHER`; live row `inputs`/`outputs` = `{}` | RLS proven for live inserts (#334) |
| Denial of service | Sink panic is caught; local-dev continues without credentials | Production availability SLO |
| Elevation of privilege | No `SupabaseSink` export; no service-role in this crate | Least-privilege live role |

## SRE

- Symptom, not cause: missing remote traces is expected until a feature-gated HTTP sink exists.
- Page-worthy: not defined. No pager, no SLA.
- Fail-closed: deployed gap lock and deployed control-plane unavailability.
- Fail-open (local-dev only): control-plane unavailability must not block kernel audit.
- Retention/evidence: document in #334 before writes; default 30 days unless incident freeze.

## Saela mapping

Saela is treated as an ethics/oversight lens, not a deployed runtime.

| Saela concern | Mapping |
| --- | --- |
| Do not overclaim | Schema ≠ integration |
| Minimize harm from logs | Empty `inputs`/`outputs` in the mapped row; no prompt persistence |
| Human-legible reasons | Stable `GAIA_*` codes, not free-form model text |
| Reversible change | Local code/docs only; no live write |
| Child/constitution data | `constitution_articles` stays server-only |

Academic honesty rule: a passing local test is evidence of the local adapter, not of production telemetry.
