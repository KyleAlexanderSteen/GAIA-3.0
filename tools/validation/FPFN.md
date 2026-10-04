# GAIA 3.0 FP/FN Conformance Framework

Issue #1701 provides reusable conformance primitives for safeguards and detection systems.

## Core semantics

```
REAL CONDITION → DETECT
NO REAL CONDITION → DO NOT INVENT CONDITION
UNKNOWN → UNKNOWN
```

The framework evaluates an expected outcome against an explicitly supplied actual classification. It never infers ground truth from the classifier itself.

## Outcomes

- `TRUE_POSITIVE`: expected POSITIVE, actual POSITIVE.
- `TRUE_NEGATIVE`: expected NEGATIVE, actual NEGATIVE.
- `FALSE_POSITIVE`: expected NEGATIVE, actual POSITIVE.
- `FALSE_NEGATIVE`: expected POSITIVE, actual NEGATIVE.
- `UNKNOWN_CORRECT`: expected UNKNOWN, actual UNKNOWN.
- `UNKNOWN_MISMATCH`: expected UNKNOWN, actual POSITIVE or NEGATIVE.

UNKNOWN is intentionally not folded into either the positive or negative class.

## Fixture categories

Fixtures may be labeled for auditability. Common categories include:

- positive
- negative
- boundary
- conflicting
- missing
- stale
- noisy
- adversarial
- unknown

A consuming test suite can require the categories appropriate to its detection problem.

## Usage

From the repository root:

```bash
python -m unittest tools/validation/tests/test_fpfn.py
```

A fixture is explicit:

```python
Fixture(
    name="insufficient-evidence",
    expected=ExpectedOutcome.UNKNOWN,
    actual=ExpectedOutcome.UNKNOWN,
    category="unknown",
)
```

Conformance can then be asserted with:

```python
assert_conforming(fixtures)
```

## Architectural boundary

This framework is a validation utility, not an authority mechanism.

Passing conformance:

- does not prove implementation,
- does not grant authorization,
- does not establish consciousness,
- does not establish causation beyond the fixture's declared expected outcome,
- does not replace CI or independent verification.

It should be used underneath domain-specific safeguards such as #1691, #1692, #1699, and future cognitive-layer assessments such as #1703.

## Required discipline

Domain tests should explicitly model:

1. a condition that should be detected,
2. a condition that should not be detected,
3. boundary or ambiguous evidence,
4. insufficient/conflicting evidence where applicable,
5. expected classification,
6. observed classification,
7. resulting FP/FN/UNKNOWN conformance status.

The framework exists to make false positives, false negatives, and unresolved uncertainty first-class test results rather than incidental observations.
