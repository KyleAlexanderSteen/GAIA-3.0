# GAIA Verification & Promotion Pipeline

Issue: #1539

## Purpose

GAIA must distinguish a build that exists from a build that has evidence supporting a declared claim.

The verification pipeline is a staged evidence model. A stage transition records that the evidence required for that transition was satisfied for an exact artifact under a declared scope. It does not establish universal correctness, absence of unknowns, or truth outside the tested scope.

## Canonical lifecycle

1. SOURCE_IDEA
2. PROVENANCE
3. SPECIFICATION
4. PREFLIGHT
5. COMPONENT_TESTING
6. INTEGRATION
7. SYSTEM_VERIFICATION
8. ADVERSARIAL_FAILURE_TESTING
9. SHADOW
10. CANARY
11. RELEASE_CANDIDATE
12. VERIFIED_BUILD
13. CONTINUOUS_OPERATIONAL_VERIFICATION

The machine-readable stage registry is `gaia-spec/verification/promotion-stages.yaml`.
Promotion records use `gaia-spec/verification/promotion-record.schema.yaml`.

## State model

| Outcome | Meaning | Promotion |
|---|---|---|
| PASS | Required evidence for the declared transition is satisfied | May advance if all gates are satisfied |
| HOLD | Evidence is incomplete or a human decision is pending | Does not advance |
| FAIL | A required condition was not satisfied | Does not advance |
| UNKNOWN | Available evidence is insufficient to determine the condition | Does not advance |

UNKNOWN is intentionally different from FAIL: lack of evidence must not be converted into either a success claim or a failure claim.

## Evidence binding

Every promotion record binds evidence to:

- repository;
- exact source/build commit;
- declared scope;
- execution environment;
- assumptions;
- limitations;
- known unknowns.

A report saying verified is not evidence that verification occurred. The underlying test, run, artifact, or other evidence reference must be recorded.

## Stage gates

### SOURCE_IDEA
Entry: a proposal, artifact, or claim has an identifiable origin.
Exit: origin, scope, and intended outcome are recorded.
Minimum evidence: source/reference, origin metadata, scope statement.

### PROVENANCE
Entry: the source can be traced to an exact revision.
Exit: commit identity and relevant inputs/transformations are recorded; integrity identifiers are available when applicable.
Minimum evidence: commit SHA plus relevant input/reference identifiers.

### SPECIFICATION
Entry: intended behavior and constraints are written down.
Exit: acceptance criteria, relevant invariants/refusals, assumptions, and unknowns are explicit.
Minimum evidence: specification/contract and acceptance criteria.

### PREFLIGHT
Entry: candidate is ready for validation.
Exit: branch/base state, structural checks, formatting, dependency checks, and applicable tests have been evaluated; blocking findings are resolved.
Minimum evidence: preflight run identifier and exact commit SHA.

GAIA already has a read-only preflight gate covering branch synchronization, known merge hotspots, missing Rust modules, rustfmt, workspace Clippy, and workspace tests. See `docs/gaia-preflight.md`.

### COMPONENT_TESTING
Entry: component has executable tests for specified behavior.
Exit: applicable component tests pass and their scope/environment are recorded.
Minimum evidence: test command/run and result.

### INTEGRATION
Entry: candidate can be exercised through real component interfaces.
Exit: applicable integration/end-to-end checks pass and interface assumptions are recorded.
Minimum evidence: integration run and result.

### SYSTEM_VERIFICATION
Entry: system-level requirements can be exercised.
Exit: declared requirements and invariants have reproducible evidence for the stated scope.
Minimum evidence: system verification report, invariant results, environment metadata.

### ADVERSARIAL_FAILURE_TESTING
Entry: risk-appropriate hostile, malformed, boundary, or failure-injection scenarios are defined.
Exit: applicable scenarios execute; expected refusal, containment, and recovery behavior is observed; unresolved critical failures block advancement.
Minimum evidence: adversarial/failure test run and failure disposition.

### SHADOW
Entry: candidate can run without authoritative production effect.
Exit: shadow execution is isolated and observed behavior is compared with expected/reference behavior.
Minimum evidence: shadow run, comparison result, isolation/rollback evidence.

### CANARY
Entry: controlled operational exposure has explicit human authorization.
Exit: exposure scope, monitoring thresholds, and rollback trigger are defined and observed within limits.
Minimum evidence: authorization, exposure scope, monitoring, rollback readiness.

### RELEASE_CANDIDATE
Entry: exact release artifact is assembled.
Exit: artifact identity is fixed, required release-candidate gates pass, and limitations/unknowns remain visible.
Minimum evidence: artifact digest and promotion record.

### VERIFIED_BUILD
Entry: exact artifact has completed the declared verification gates.
Exit: all required evidence is recorded, no blocking condition remains within declared scope, and the record states what was and was not verified.
Minimum evidence: decision record plus complete evidence references.

Compilation, a green CI run, elapsed time, or narrative confidence alone cannot establish VERIFIED_BUILD.

### CONTINUOUS_OPERATIONAL_VERIFICATION
Entry: promoted artifact is operating under active monitoring.
Exit: operational verification, drift/regression response, incident handling, and rollback/demotion remain active.
Minimum evidence: telemetry, drift/regression checks, and incident/rollback records.

## Relationship to existing GAIA gates

- Preflight is the PREFLIGHT gate.
- Definition-of-done remains the PR-level contract for the existing SPECIFICATION → IMPLEMENTATION → TEST → INTEGRATION → VERIFICATION → OPERATIONAL vocabulary.
- Provenance and epistemic state provide evidence context rather than being replaced by this pipeline.
- Audit records the decision and its supporting evidence.
- Human authorization is mandatory where promotion changes operational exposure.
- Rollback remains part of any operational promotion.

## Non-negotiable rules

1. No stage promotion without evidence.
2. No evidence may be detached from the exact artifact/commit it describes.
3. HOLD, FAIL, and UNKNOWN are first-class outcomes.
4. Unknown evidence is not silently treated as success or failure.
5. A verification report is not proof of verification by itself.
6. CI is evidence, not a universal correctness oracle.
7. A higher stage does not erase limitations or known unknowns.
8. Operational exposure requires explicit authorization and rollback readiness.
9. Promotion decisions must be auditable and reproducible from their recorded evidence.
10. Exceptions must themselves be recorded as evidence-bearing, human-authorized decisions.

## Initial implementation boundary

This issue establishes the canonical model and machine-readable contracts. It intentionally does not claim that all thirteen stages are already automated or that every stage has runtime enforcement.

The next implementation work can wire existing preflight, tests, provenance, audit, and PR checks into these stage records without falsely declaring the full pipeline operational.