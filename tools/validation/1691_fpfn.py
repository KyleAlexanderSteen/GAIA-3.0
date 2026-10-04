#!/usr/bin/env python3
"""Validate #1691 runtime FP/FN evidence with the shared #1701 contract."""

from __future__ import annotations

import re
import sys

from fpfn import (
    ConformanceResult,
    ExpectedOutcome,
    Fixture,
    evaluate,
    require_categories,
)

LINE = re.compile(r"FPFN\|(?P<name>[^|]+)\|(?P<expected>[^|]+)\|(?P<actual>[^\r\n]+)")

EXPECTED = {
    "Stable": ExpectedOutcome.NEGATIVE,
    "RisingLoad": ExpectedOutcome.NEGATIVE,
    "SustainedDegradation": ExpectedOutcome.NEGATIVE,
    "HighLoad": ExpectedOutcome.POSITIVE,
    "ConflictingEvidence": ExpectedOutcome.UNKNOWN,
    "ExplicitStop": ExpectedOutcome.POSITIVE,
}

STATE_TO_OUTCOME = {
    "Continue": ExpectedOutcome.NEGATIVE,
    "Slow": ExpectedOutcome.NEGATIVE,
    "Pause": ExpectedOutcome.NEGATIVE,
    "Stop": ExpectedOutcome.POSITIVE,
    "Unknown": ExpectedOutcome.UNKNOWN,
}


def main() -> int:
    output = sys.stdin.read()
    fixtures = []

    for match in LINE.finditer(output):
        name = match.group("name")
        expected_state = match.group("expected")
        actual_state = match.group("actual")
        if name not in EXPECTED:
            continue
        if expected_state not in STATE_TO_OUTCOME or actual_state not in STATE_TO_OUTCOME:
            raise SystemExit(
                f"invalid #1691 FP/FN state evidence for {name}: "
                f"{expected_state!r}/{actual_state!r}"
            )
        fixtures.append(
            Fixture(
                name=name,
                expected=EXPECTED[name],
                actual=STATE_TO_OUTCOME[actual_state],
                category=(
                    "unknown"
                    if EXPECTED[name] is ExpectedOutcome.UNKNOWN
                    else "positive"
                    if EXPECTED[name] is ExpectedOutcome.POSITIVE
                    else "negative"
                ),
                rationale=f"Rust runtime state: expected={expected_state}, actual={actual_state}",
            )
        )

    if len(fixtures) != len(EXPECTED):
        missing = sorted(set(EXPECTED) - {fixture.name for fixture in fixtures})
        raise SystemExit("missing #1691 FP/FN evidence: " + ", ".join(missing))

    require_categories(fixtures, "positive", "negative", "unknown")
    results = evaluate(fixtures)

    false_positive = sum(result is ConformanceResult.FALSE_POSITIVE for result in results)
    false_negative = sum(result is ConformanceResult.FALSE_NEGATIVE for result in results)
    unknown_mismatch = sum(
        result is ConformanceResult.UNKNOWN_MISMATCH for result in results
    )

    print("Shared #1701 FP/FN conformance results for #1691:")
    for fixture, result in zip(fixtures, results):
        print(f"- {fixture.name}: {result.value}")
    print(f"False positives: {false_positive}")
    print(f"False negatives: {false_negative}")
    print(f"UNKNOWN mismatches: {unknown_mismatch}")

    if false_positive or false_negative or unknown_mismatch:
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
