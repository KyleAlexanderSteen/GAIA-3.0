# PR issue lifecycle governance

Tracking issue: #1624

## Canonical rule

**One issue = one tracked unit of work.**

Every implementation pull request must declare which tracked issue owns the work. `Refs` is the default for partial or foundational work. `Closes` is reserved for work that reaches OPERATIONAL and satisfies the issue acceptance criteria.

GitHub interprets `Closes`, `Fixes`, and `Resolves` as closing keywords when a pull request is merged into the default branch. GAIA therefore treats all of those forms as completion claims, not merely references.

## Relationship classes

| Relationship | Meaning | Merge effect |
| --- | --- | --- |
| `Refs #N` | Partial, foundational, or intentionally incomplete work | Issue remains open |
| `Closes #N` | Complete issue; OPERATIONAL evidence is present | GitHub closes the issue on merge |
| `Fixes #N` / `Resolves #N` | Equivalent GitHub completion claim | GitHub closes the issue on merge |
| `Duplicate of #N` | Work is already represented by another canonical issue | Duplicate issue is consolidated; no second implementation |

## Before implementation

1. Search existing open issues for overlapping scope.
2. Identify the canonical issue.
3. Search open pull requests for active implementation work against that issue.
4. If work is intentionally split, document the relationship and keep the issue open with `Refs`.
5. If an issue is a duplicate, mark and close it rather than implementing it independently.

## Before opening a PR

- The PR body contains an explicit issue relationship.
- The relationship uses `Refs` unless the PR truly reaches OPERATIONAL.
- Closing claims have OPERATIONAL checked and evidence linked.
- The work does not collide with another active implementation PR for the same issue unless the split is intentional and documented.

## After merge

The lifecycle workflow checks every merged PR relationship against the observed GitHub issue state: `Closes`/`Fixes`/`Resolves` must produce a closed issue, while `Refs` must leave the issue open. A deterministic verifier is unit-tested and also exercised by a manual end-to-end workflow self-test. If the observed state does not match the declared relationship, CI fails instead of silently treating the work as complete.

## Automated work-governance reports

The lifecycle workflow now emits two machine-readable artifacts for active pull requests:

- **Readiness report** — explicit DoD stages, supplied validation evidence, blockers, and a promotion result. The result can be **BLOCKED**, **INCOMPLETE**, or **READY FOR HUMAN APPROVAL**; it never grants authority.
- **Overlap report** — shared issue references and changed-file overlap with other active PRs. Candidates are surfaced for human review; the automation never declares an unrelated issue a duplicate.

Historical work can be classified with `tools/agent-skills/work_history_audit.py`. It is intentionally mutation-free: unfinished historical work produces a recovery recommendation instead of reopening an old PR.

The reports are commit-bound where an evaluated SHA is supplied and are suitable as inputs to the broader verification/promotion contracts in #1539 and #1542.

## Scope boundary

This governance layer prevents bookkeeping and duplicate-work failures; it does not decide whether an issue's technical acceptance criteria are actually satisfied. Definition of Done remains the source of truth for stage evidence.

Related: #1319, #1537, #1539, #1542, #1603, #1624
