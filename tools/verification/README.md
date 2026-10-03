# Verification Simulation Gate

This package provides a deterministic, offline simulation layer for GAIA's pre-deployment verification work.

## What it does

- Runs versioned JSON scenarios.
- Uses an explicit seed for reproducible event ordering.
- Injects controlled failures.
- Checks declared invariants after execution.
- Emits PASS or FAIL evidence suitable for CI.
- Never performs production actions, writes repository state, or auto-resolves conflicts.

## Commands

    python tools/verification/simulation_gate.py
    python tools/verification/simulation_gate.py --json
    python -m unittest tools/verification/tests/test_simulation_gate.py

A failing result preserves the scenario id and seed so the same case can be replayed after repair.

## Scope

This is the first executable slice of issues #1567-#1577: versioned scenarios, deterministic replay, controlled failure injection, adversarial boundary checks, recovery evidence, and a gate that can be called from preflight. Multi-agent orchestration and production deployment remain separate stages.
