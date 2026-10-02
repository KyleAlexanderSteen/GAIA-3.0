# Definition of done

Tracking issue: #1319. Checker: `tools/agent-skills/dod_check.py`. Template: `.github/PULL_REQUEST_TEMPLATE.md`.

This is the rule that decides when GAIA is allowed to say a piece of work is finished. It exists because the project has repeatedly had work that looked finished (a command that prints success, a test suite nobody runs in CI, a document describing something that does not exist) and was counted as done. Every contributor, human or agent, follows it.

## The one-sentence rule

A pull request must state the highest of six stages it actually reaches, link evidence for that stage, and an issue may only be closed as complete when there is evidence for the last stage, OPERATIONAL.

## The six stages

Each stage includes the ones before it. Claim only what you can link evidence for.

| Stage | Meaning | Evidence to link | Typical false claim |
|---|---|---|---|
| SPECIFICATION | The intended behavior is written down | The spec, RFC or doc file | A doc that describes behavior the code does not have |
| IMPLEMENTATION | Code for it exists on the branch | The source file | Stubs, placeholders, or functions that return success without doing the work |
| TEST | A test exercises it, and the test fails without the change | The test name and a run | Tests that mock the thing under test, or call APIs that do not exist |
| INTEGRATION | It runs wired to the real neighboring components, not mocks | A test or CI run that uses the real parts | Unit tests passing while the feature is not connected to anything |
| VERIFICATION | CI passes on a clean checkout | The CI run link | Tests that only pass on one machine or in one sandbox |
| OPERATIONAL | A user can run it end to end and the run is recorded in an audit receipt | The run and its receipt | A demo, a screenshot, or a command that prints "ok" |

Two consequences that surprise people:

- A documents-only pull request reaches SPECIFICATION and nothing higher, however good the documents are.
- Passing tests on a feature that is not wired into the system reaches TEST, not INTEGRATION.

## Refs versus Closes

- `Refs #N` links an issue without closing it. This is the default.
- `Closes #N` is allowed only when OPERATIONAL is checked, because merging the pull request closes the issue and a closed issue is read as "finished".
- Most pull requests should say `Refs`. Large issues are finished by a series of pull requests, and the last one, with operational evidence, may use `Closes`.

GitHub also closes issues on `Fixes #N` and `Resolves #N` (and their variants). The current checker does not look for those (see Known gaps), so do not use them: use `Refs`.

## How to fill in the pull request body

The checker reads the body of the pull request, finds the section headed `## Stage reached`, and requires at least one stage written as a checked checkbox. The format is exact:

```markdown
Refs #1341

## Stage reached
- [x] SPECIFICATION
- [x] IMPLEMENTATION
- [x] TEST
- [ ] INTEGRATION
- [ ] VERIFICATION
- [ ] OPERATIONAL

Evidence:
- TEST: gaia-spec/symbology-v0/test_symbology.py, run output in results.json
```

Format rules, each verified against the checker (see below):

- The heading must start with `## Stage reached` (exactly two `#`). `### Stage reached` is not found.
- Each stage is a line starting `- [x] ` followed by the stage name in capitals. `[x]` or `[X]` both work.
- Stage names are case-sensitive: `- [x] test` does not count.
- `* [x] TEST` does not count; only the `-` bullet does.
- Bold prose such as `**Implementation + test.**` does not count, even if it is true. This is the mistake that made the check fail on PR #1342.
- Checked boxes placed after the next `##` heading are outside the section and do not count.

## What the checker actually does

The script is about 20 lines. It applies two rules to the pull request body:

1. A `## Stage reached` section must exist and contain at least one checked stage.
2. If a line starts with `Closes #<number>`, then OPERATIONAL must be checked.

That is all. It does not read the evidence, and it does not look at the code.

### Behavior table

Each row was run against the script at the blob `518f2cc` on main. "Fails" means the check exits with an error.

| Pull request body | Result |
|---|---|
| `Refs #1` and `- [x] SPECIFICATION` | Passes |
| `Closes #1` and `- [x] TEST` | Fails: Closes requires OPERATIONAL |
| `Closes #1` and `- [x] OPERATIONAL` | Passes |
| `Refs #1` and bold prose instead of a checkbox | Fails: no stage is checked |
| `Refs #1` and `* [x] TEST` | Fails: no stage is checked |
| `Refs #1` and `- [x] test` (lowercase) | Fails: no stage is checked |
| `### Stage reached` instead of `##` | Fails: section missing |
| No section at all | Fails: section missing |
| Checked box after a later `##` heading | Fails: no stage is checked |
| Unfilled template (`Closes #`, nothing checked) | Fails: no stage is checked |

## Known gaps

The same test run showed that the checker passes several bodies that break the intent of the rule. These are documented here so nobody assumes the check is stronger than it is. They are not fixed in this pull request.

| Body | Result | Why it matters |
|---|---|---|
| `Fixes #1` and `- [x] SPECIFICATION` | Passes | GitHub closes the issue on merge, but OPERATIONAL is not required. This bypasses the rule. |
| `Resolves #1` and `- [x] SPECIFICATION` | Passes | Same bypass. |
| `This closes #1` mid-line | Passes | Same bypass, because only a line starting with `Closes` is detected. |
| Only `- [x] OPERATIONAL` checked, nothing else, no evidence | Passes | Claims the top stage with no lower stages and no evidence. |
| All six stages checked, `Evidence:` left empty | Passes | Evidence is never read. |
| `- [x]TEST` (no space after the box) | Passes | Harmless, but shows the matching is loose. |

The important point: **the check proves the pull request body is well formed, not that the work is done.** The stages are honest only if the author and reviewer are honest. Review must look at the evidence links, not the boxes.

## Planned hardening (not part of this pull request)

Suggested as a separate code pull request with tests, which would itself reach TEST:

1. Detect all closing keywords GitHub recognizes (close, closes, closed, fix, fixes, fixed, resolve, resolves, resolved) anywhere in the body, and apply the OPERATIONAL requirement to all of them.
2. Require the highest checked stage to have every lower stage checked.
3. Require a non-empty `Evidence:` line for the highest checked stage, and require at least one link or file path in it.
4. Add `tools/agent-skills/test_dod_check.py` with every row of the tables above as a test case.
5. Later: check that linked evidence exists (file present on the branch, CI run is green). This is what would make the check mean something.

## Worked examples

### A prototype with tests that is not wired in (PR #1342, symbology)

- Checked: SPECIFICATION, IMPLEMENTATION, TEST.
- Unchecked: INTEGRATION (not connected to anything), VERIFICATION (not in CI), OPERATIONAL.
- Uses `Refs #1341`, so the issue stays open. Correct.

### A documents-only pull request (the template change in #1320)

- Checked: SPECIFICATION only. Correct, even though the change affects every future pull request.

### A stub command that prints success

- Stage is at most IMPLEMENTATION, and arguably not even that if the function does no work. It must not be described as done, and the stubs-must-fail-loudly rule (#1304) says it should return an error instead of reporting success.

### A feature finished by three pull requests

1. Spec and design: SPECIFICATION, `Refs`.
2. Code and unit tests: up to TEST, `Refs`.
3. Wiring, CI, end-to-end run with receipt: up to OPERATIONAL, may use `Closes`.

## Rules for agents

- Read the files you change. Do not invent types or APIs.
- Do not check a stage you have not demonstrated. If unsure, check a lower one and say what is missing.
- If the checker fails, fix the body or the work. Never edit the checker to make a failing body pass inside the same pull request.
- Report the stage plainly in your summary, including what is unchecked.

## Open questions

- What counts as an "audit receipt" for OPERATIONAL is not yet specified here. It needs its own definition tied to the receipt format.
- Whether the checker should become a required status check on main is a maintainer decision.
