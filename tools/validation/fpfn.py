"""Reusable false-positive/false-negative conformance primitives for GAIA 3.0 (#1701).

The framework deliberately keeps UNKNOWN distinct from positive/negative results.
It validates classifier behavior against an explicit expected outcome; it never
infers truth from the classifier output.
"""
from __future__ import annotations

from dataclasses import dataclass
from enum import Enum


class ExpectedOutcome(str, Enum):
    POSITIVE = "POSITIVE"
    NEGATIVE = "NEGATIVE"
    UNKNOWN = "UNKNOWN"


class ConformanceResult(str, Enum):
    TRUE_POSITIVE = "TRUE_POSITIVE"
    TRUE_NEGATIVE = "TRUE_NEGATIVE"
    FALSE_POSITIVE = "FALSE_POSITIVE"
    FALSE_NEGATIVE = "FALSE_NEGATIVE"
    UNKNOWN_CORRECT = "UNKNOWN_CORRECT"
    UNKNOWN_MISMATCH = "UNKNOWN_MISMATCH"


@dataclass(frozen=True)
class Fixture:
    name: str
    expected: ExpectedOutcome
    actual: ExpectedOutcome
    category: str = "baseline"
    rationale: str = ""

    def evaluate(self) -> ConformanceResult:
        if self.expected is ExpectedOutcome.UNKNOWN:
            return (
                ConformanceResult.UNKNOWN_CORRECT
                if self.actual is ExpectedOutcome.UNKNOWN
                else ConformanceResult.UNKNOWN_MISMATCH
            )
        if self.expected is ExpectedOutcome.POSITIVE:
            return (
                ConformanceResult.TRUE_POSITIVE
                if self.actual is ExpectedOutcome.POSITIVE
                else ConformanceResult.FALSE_NEGATIVE
            )
        return (
            ConformanceResult.TRUE_NEGATIVE
            if self.actual is ExpectedOutcome.NEGATIVE
            else ConformanceResult.FALSE_POSITIVE
        )


def evaluate(fixtures: list[Fixture]) -> list[ConformanceResult]:
    """Evaluate fixtures without collapsing UNKNOWN into a binary result."""
    return [fixture.evaluate() for fixture in fixtures]


def assert_conforming(fixtures: list[Fixture]) -> None:
    """Raise AssertionError unless every fixture has the expected outcome."""
    failures = [
        (fixture.name, fixture.evaluate().value)
        for fixture in fixtures
        if fixture.evaluate()
        not in {
            ConformanceResult.TRUE_POSITIVE,
            ConformanceResult.TRUE_NEGATIVE,
            ConformanceResult.UNKNOWN_CORRECT,
        }
    ]
    if failures:
        detail = ", ".join(f"{name}={result}" for name, result in failures)
        raise AssertionError(f"FP/FN conformance failure: {detail}")


def require_categories(fixtures: list[Fixture], *categories: str) -> None:
    """Require explicit coverage of named fixture categories."""
    present = {fixture.category for fixture in fixtures}
    missing = [category for category in categories if category not in present]
    if missing:
        raise AssertionError(
            "missing required FP/FN fixture categories: " + ", ".join(missing)
        )
