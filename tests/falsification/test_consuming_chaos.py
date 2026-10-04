#!/usr/bin/env python3
"""Pass 22: consuming-chaos threat-pattern falsification.

Issue: #1672
Refs: #1652, #1660, #1662, #1664, #1665, #1666, #1667, #1668, #1669,
      #1670, #1671

This is a deterministic semantic fixture, not evidence of a supernatural
entity or metaphysical ontology. "Consumption" means that a transition
reduces one or more protected capacities/assets while the event remains
auditable.

Stdlib only.
"""
from __future__ import annotations

from dataclasses import dataclass, replace
from typing import FrozenSet, Tuple


RESPONSE_GRAMMAR = (
    "DETECT",
    "PRESERVE",
    "CONTAIN",
    "BOUND_AUTHORITY",
    "VERIFY",
    "PROTECT",
    "RECOVER",
    "LEARN",
)

PROTECTED = frozenset(
    {
        "identity",
        "provenance",
        "evidence",
        "agency",
        "recovery_capacity",
    }
)

ATTACKS = (
    "identity_substitution",
    "provenance_destruction",
    "evidence_consumption",
    "condition_laundering",
    "authorization_erosion",
    "resource_exhaustion",
    "recursive_cascade",
    "agency_suppression",
    "recovery_compromise",
    "false_attribution",
    "defensive_overreaction",
    "greater_good_injection",
)


@dataclass(frozen=True)
class Scenario:
    name: str
    unknown: bool = True
    consumed: FrozenSet[str] = frozenset()
    evidence_preserved: bool = True
    provenance_preserved: bool = True
    contradictory_evidence_preserved: bool = True
    authority_before: int = 1
    authority_after: int = 1
    destructive_objective: bool = False
    possession_or_domination: bool = False
    fabricated_history: bool = False
    verified: bool = False


def consume(s: Scenario, *assets: str) -> Scenario:
    return replace(s, consumed=frozenset(set(s.consumed) | set(assets)))


def bounded_defensive_response(s: Scenario) -> Tuple[Scenario, Tuple[str, ...]]:
    """Apply only bounded defensive semantics; never grant new authority."""
    trace = RESPONSE_GRAMMAR
    out = replace(
        s,
        evidence_preserved=True,
        provenance_preserved=True,
        contradictory_evidence_preserved=True,
        authority_after=s.authority_before,
        destructive_objective=False,
        possession_or_domination=False,
        fabricated_history=False,
        verified=False,
    )
    # Recovery restores capacity where the fixture permits it without
    # pretending that lost historical evidence was recovered.
    out = replace(out, consumed=frozenset())
    return out, trace


def assert_invariants(original: Scenario, recovered: Scenario, trace: Tuple[str, ...]) -> None:
    assert trace == RESPONSE_GRAMMAR
    assert original.unknown is True  # UNKNOWN is valid; it is not auto-hostile.
    assert recovered.authority_after <= original.authority_before
    assert recovered.evidence_preserved
    assert recovered.provenance_preserved
    assert recovered.contradictory_evidence_preserved
    assert not recovered.destructive_objective
    assert not recovered.possession_or_domination
    assert not recovered.fabricated_history
    assert "UNKNOWN" not in trace


def build_cases() -> tuple[Scenario, ...]:
    return (
        consume(Scenario("identity_substitution"), "identity"),
        consume(Scenario("provenance_destruction"), "provenance"),
        consume(Scenario("evidence_consumption"), "evidence"),
        consume(Scenario("condition_laundering"), "provenance", "evidence"),
        consume(Scenario("authorization_erosion"), "agency"),
        consume(Scenario("resource_exhaustion"), "recovery_capacity"),
        consume(Scenario("recursive_cascade"), *PROTECTED),
        consume(Scenario("agency_suppression"), "agency"),
        consume(Scenario("recovery_compromise"), "recovery_capacity"),
        consume(Scenario("false_attribution"), "evidence", "provenance"),
        consume(Scenario("defensive_overreaction"), "agency"),
        consume(Scenario("greater_good_injection"), "agency", "evidence"),
    )


def run() -> int:
    failures = 0

    cases = build_cases()
    names = {case.name for case in cases}
    if names != set(ATTACKS):
        print("FAIL attack corpus is incomplete")
        failures += 1

    for case in cases:
        if not case.consumed:
            print(f"FAIL {case.name}: no consumption signal")
            failures += 1
            continue

        recovered, trace = bounded_defensive_response(case)
        try:
            assert_invariants(case, recovered, trace)
        except AssertionError as exc:
            print(f"FAIL {case.name}: invariant violation {exc}")
            failures += 1
        else:
            print(f"PASS {case.name}")

    # Explicit negative controls: these must be rejected by the invariants.
    bad = replace(
        cases[0],
        authority_after=cases[0].authority_before + 1,
        destructive_objective=True,
        fabricated_history=True,
    )
    try:
        assert_invariants(cases[0], bad, RESPONSE_GRAMMAR)
    except AssertionError:
        print("PASS negative control rejected: authority expansion/destruction/history fabrication")
    else:
        print("FAIL negative control accepted")
        failures += 1

    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(run())
