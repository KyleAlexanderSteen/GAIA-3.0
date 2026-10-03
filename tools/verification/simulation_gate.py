#!/usr/bin/env python3
"""Deterministic, offline verification simulation gate."""
from __future__ import annotations
import argparse, hashlib, json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

@dataclass
class SimState:
    executed: list[str] = field(default_factory=list)
    refused: list[str] = field(default_factory=list)
    failures: list[str] = field(default_factory=list)
    recovered: bool = False

def stable_event_stream(events: list[dict[str, Any]], seed: int) -> list[dict[str, Any]]:
    keyed = []
    for index, event in enumerate(events):
        raw = f"{seed}:{index}:{json.dumps(event, sort_keys=True)}".encode()
        keyed.append((hashlib.sha256(raw).hexdigest(), index, event))
    return [event for _, _, event in sorted(keyed)]

def apply_event(state: SimState, event: dict[str, Any]) -> None:
    kind = event.get("kind"); name = str(event.get("name", kind or "unnamed"))
    if kind == "execute":
        if event.get("authorized", True): state.executed.append(name)
        else: state.refused.append(name)
    elif kind == "refuse": state.refused.append(name)
    elif kind == "inject_failure": state.failures.append(name)
    elif kind == "recover": state.recovered = True
    elif kind == "noop": return
    else: raise ValueError(f"unknown event kind: {kind!r}")

def check_invariants(state: SimState, invariants: list[str]) -> list[str]:
    failures = []
    for invariant in invariants:
        if invariant == "refused_action_never_executes":
            overlap = set(state.refused) & set(state.executed)
            if overlap: failures.append(f"refused actions executed: {sorted(overlap)}")
        elif invariant == "failure_is_recorded":
            if state.failures is None: failures.append("failure state is unavailable")
        elif invariant == "recovery_is_explicit":
            if state.failures and not state.recovered: failures.append("failure occurred without explicit recovery")
        else: failures.append(f"unknown invariant: {invariant}")
    return failures

def run_scenario(scenario: dict[str, Any], seed: int | None = None) -> dict[str, Any]:
    actual_seed = int(scenario.get("seed", 0) if seed is None else seed)
    state = SimState(); errors = []
    try:
        for event in stable_event_stream(list(scenario.get("events", [])), actual_seed): apply_event(state, event)
        errors.extend(check_invariants(state, list(scenario.get("invariants", []))))
    except Exception as exc:
        errors.append(f"runner error: {type(exc).__name__}: {exc}")
    return {"schema_version":"1.0","scenario_id":str(scenario["id"]),"seed":actual_seed,"outcome":"PASS" if not errors else "FAIL","errors":errors,"state":{"executed":state.executed,"refused":state.refused,"failures":state.failures,"recovered":state.recovered}}

def load_scenarios(path: Path):
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, list): raise ValueError("scenario file must contain a JSON array")
    return payload

def main() -> int:
    p = argparse.ArgumentParser(); p.add_argument("--scenarios", type=Path, default=Path("tools/verification/scenarios.json")); p.add_argument("--scenario"); p.add_argument("--seed", type=int); p.add_argument("--json", action="store_true"); a=p.parse_args()
    scenarios=load_scenarios(a.scenarios)
    if a.scenario:
        scenarios=[s for s in scenarios if s.get("id")==a.scenario]
        if not scenarios: raise SystemExit(f"scenario not found: {a.scenario}")
    results=[run_scenario(s,a.seed) for s in scenarios]
    if a.json: print(json.dumps(results,indent=2,sort_keys=True))
    else:
        for r in results:
            print(f"[{r["outcome"]}] {r["scenario_id"]} seed={r["seed"]}")
            for e in r["errors"]: print(f"  - {e}")
    return 0 if all(r["outcome"]=="PASS" for r in results) else 1

if __name__ == "__main__": raise SystemExit(main())
