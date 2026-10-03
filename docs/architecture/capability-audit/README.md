# GAIA 3.0 Capability Audit Foundation

This directory defines the canonical capability vocabulary for the OS + AI completeness audit.

## Audit rule

A capability name, interface, stub, documentation entry, or placeholder is **not** evidence of implementation.

A capability is considered **implemented** only when the repository can provide executable evidence appropriate to that capability, including tests and/or runtime verification.

The audit therefore separates:

```
CAPABILITY → PURPOSE → GAIA SURFACE → IMPLEMENTATION → TEST → EVIDENCE → STATUS
```

## Canonical status vocabulary

- `implemented` — executable implementation exists and has appropriate verification evidence.
- `partial` — some required behavior exists, but the capability is incomplete or materially bounded.
- `prototype` — executable experimental implementation exists but is not yet production-ready.
- `designed_documented` — architecture/specification exists without sufficient executable implementation.
- `missing` — required capability has no adequate implementation.
- `intentionally_excluded` — explicitly outside GAIA's applicable scope, with rationale.

## Evidence rule

Interfaces and declarations must never be promoted to implementation claims automatically.

In particular:

- a module name is not an implementation;
- an API endpoint is not proof of the underlying capability;
- a stub is not a successful implementation;
- documentation is not runtime evidence;
- a test that only checks an interface exists is not sufficient proof of behavior;
- model/provider capabilities must be distinguished from GAIA control-plane capabilities.

## Domains

The canonical vocabulary is divided into:

1. **OS substrate** — primitives required to operate and manage computing resources.
2. **AI/agent substrate** — primitives required to perceive, represent, reason, learn, plan, communicate, use tools, and operate within bounded agency.
3. **GAIA control plane** — identity, authorization, provenance, simulation, verification, recovery, promotion, and governance mechanisms that bind the OS and AI layers together.

The inventories are intentionally capability-oriented rather than product-oriented. GAIA is not required to reproduce every interface or feature of another operating system or AI platform.

## Lifecycle

This vocabulary is the foundation for the later evidence matrix and executable completeness gate:

```
Vocabulary
  ↓
Capability matrix
  ↓
Claim → code → test → evidence mapping
  ↓
Automated preflight
  ↓
Simulation / failure injection
  ↓
Verification
  ↓
Canary
  ↓
Promotion
```

## Related issues

- #1598 — OS + AI Capability Completeness Audit
- #1599 — Fundamental operating-system capability surface
- #1600 — Fundamental AI and agent capability surface
- #1601 — Claim-to-code-to-test-to-evidence completeness matrix
- #1602 — Missing systems primitives vs optional features
- #1603 — Capability completeness in preflight and CI
