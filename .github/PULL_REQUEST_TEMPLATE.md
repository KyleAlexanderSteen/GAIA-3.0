## Summary

## Issue

**Relationship (required):**

- [ ] `Refs #N` — partial/foundational work; issue remains open
- [ ] `Closes #N` — this PR fully completes the issue and reaches OPERATIONAL
- [ ] `Duplicate of #N` — duplicate tracked elsewhere; do not implement separately

Use one or more explicit issue references. **Do not use `Closes`/`Fixes`/`Resolves` unless OPERATIONAL is checked and the issue's acceptance criteria are actually complete.** `Refs` is the default for partial work.

Before opening this PR:

- [ ] I searched open issues for overlapping scope.
- [ ] I identified the canonical issue(s) this work belongs to.
- [ ] I checked for an existing active PR implementing the same issue.
- [ ] If this is intentionally split work, the split/relationship is documented above or in the PR body.

## Stage reached (definition of done, #1319)

Pick the highest stage this PR actually reaches and link the evidence. A PR that only adds or changes documents reaches SPECIFICATION and nothing higher. Full guide: `docs/process/definition-of-done.md`.

Format matters: keep the heading starting `## Stage reached`, and mark stages as `- [x] NAME` with the name in capitals. Bold text or `*` bullets are not recognized by the checker.

- [ ] SPECIFICATION: the behavior is written down (link the file)
- [ ] IMPLEMENTATION: code exists on this branch (link the file)
- [ ] TEST: a test exercises it and fails without the change (name the test)
- [ ] INTEGRATION: it runs wired to the real neighbors, not mocks (link the test or CI run)
- [ ] VERIFICATION: CI passes on a clean checkout (link the run)
- [ ] OPERATIONAL: a user can run it end to end and the run is recorded in an audit receipt (link the run)

## Evidence

- IMPLEMENTATION: <link or test name>

An issue is only closed as complete when OPERATIONAL evidence is linked. If this PR does not reach OPERATIONAL, use `Refs #` instead of `Closes #`.

## Layer

- [ ] gaia-spec
- [ ] gaia-sdk
- [ ] gaia-kernel / sfs / memos
- [ ] gaia-orchestrator / agents / interface
- [ ] gaia-earth / gaia-gaian / gaia-sa
- [ ] docs / governance

## Moral architecture (#954)

Cite the principle(s) this change serves. See `docs/canon/moral-architecture.md`.

- [ ] Power without domination
- [ ] Autonomy without abandonment of accountability
- [ ] Knowledge without pretending certainty
- [ ] Intelligence without superiority
- [ ] Transformation without violating consent
- [ ] Stewardship without ownership of other people
- [ ] Correction without shame
- [ ] Strength without cruelty

## Agent hygiene

- [ ] I read the files I changed; no invented types
- [ ] Official repo only (`KyleAlexanderSteen/GAIA-3.0`)
- [ ] Did not strip NOTICE / license / author
- [ ] Tests call APIs that exist on this branch

## Checklist

- [ ] No success output for unimplemented behavior
- [ ] Spec examples still validate (`python gaia-spec/tools/validate_aip.py`)
- [ ] Claim tags still valid (`python gaia-spec/tools/check_claim_tags.py`)
- [ ] No silent protocol assumptions — linked an RFC if needed
- [ ] Tests added or explained why not
- [ ] If this closes a `bug`/`defect` issue: regression lock added (`gaia-integrity/tests/regression.rs`, see `tests/regression/README.md`)
- [ ] Signed-off commit (`git commit -s`)
- [ ] Did not promote a prohibited claim to established
