# GAIA 3.0 Preflight Validation

Issue #1700 adds a deterministic local pre-PR validator.

## CLI

From the repository root:

```bash
python tools/agent-skills/preflight.py --body pr-body.md --repo-root .
```

Without `--repo-root`, the validator runs the existing Definition-of-Done checks only. With `--repo-root`, evidence values that look like repository-relative paths are checked for existence.

URLs and human-readable test names are not treated as repository paths.

## Result semantics

`READY FOR VALIDATION` means local checks passed. It does **not** mean the change is VERIFIED, IMPLEMENTED, AUTHORIZED, or safe to merge.

CI must rerun the validator against the same commit. The validator never infers implementation, testing, verification, authority, or authorization from prose.

## Evidence boundary

A checked stage and its evidence remain separate requirements:

```markdown
## Stage reached
- [x] SPECIFICATION
- [x] IMPLEMENTATION

## Evidence
- IMPLEMENTATION: tools/agent-skills/preflight.py
```

The standalone evidence bullet is intentional. A checked stage alone is not evidence.
