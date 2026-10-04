# GAIA 3.0 Reference Implementations — #1691 / #1692

These modules are deterministic reference implementations of the two architectural
contracts. They do not infer consciousness, diagnose human health conditions, or
grant authority.

## #1691 Load / Recovery

load_recovery.py implements normalized load indicators, configurable thresholds,
explicit UNKNOWN handling, the workload state machine, legal transitions,
continuity-scope checking, and FP/FN fixtures using the shared #1701 framework.

The score is the normalized mean of declared indicators. This is a reference model,
not a clinically validated workload score. Thresholds are configuration, not
universal truths.

## #1692 Energy-Aware States

energy_states.py implements the canonical computational state machine, legal
transitions, explicit wake authorization/evidence, authorization continuity,
auditable transition records, and resource/latency measurement containers.

ResourceMetrics is a measurement contract; it does not claim empirical energy
savings until connected to a real runtime and measured.

## Validation boundary

Passing these tests establishes conformance of the reference implementation to the
declared contracts. It does not establish runtime integration, empirical
performance, consciousness, or medical validity.

Remaining integration work is tracked explicitly rather than silently claimed.
