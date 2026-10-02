## Summary

## Issue

Refs #

Use `Refs #` by default. Use `Closes #` only if OPERATIONAL is checked below. Do not use `Fixes`/`Resolves`: GitHub closes issues on those too, and the checker does not yet catch them.

## Stage reached (definition of done, #1319)

Pick the highest stage this PR actually reaches and link the evidence. A PR that only adds or changes documents reaches SPECIFICATION and nothing higher. Full guide: `docs/process/definition-of-done.md`.

Format matters: keep the heading starting `## Stage reached`, and mark stages as `- [x] NAME` with the name in capitals. Bold text or `*` bullets are not recognized by the checker.

- [ ] SPECIFICATION: the behavior is written down (link the file)
- [ ] IMPLEMENTATION: code exists on this branch (link the file)
- [ ] TEST: a test exercises it and fails without the change (name the test)
- [ ] INTEGRATION: it runs wired to the real neighbors, not mocks (link the test or CI run)
- [ ] VERIFICATION: CI passes on a clean checkout (link the run)
- [ ] OPERATIONAL: a user can run it end to end and it is recorded in an audit receipt (link the run)

Evidence:

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
- [ ] Official repo only (`R0GV3TheAvatar/GAIA-3.0`)
- [ ] Did not strip NOTICE / license / author
- [ ] Tests call APIs that exist on this branch

## Checklist

- [ ] Spec examples still validate (`python gaia-spec/tools/validate_aip.py`)
- [ ] Claim tags still valid (`python gaia-spec/tools/check_claim_tags.py`)
- [ ] No silent protocol assumptions — linked an RFC if needed
- [ ] Tests added or explained why not
- [ ] If this closes a `bug`/`defect` issue: regression lock added (`gaia-integrity/tests/regression.rs`, see `tests/regression/README.md`)
- [ ] Signed-off commit (`git commit -s`)
- [ ] Did not promote a prohibited claim to established
