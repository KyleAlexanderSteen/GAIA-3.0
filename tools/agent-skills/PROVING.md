# How we prove things work

A proposal, not a settled process. It extends the stage vocabulary from #1319 (SPECIFICATION, IMPLEMENTATION, TEST, INTEGRATION, VERIFICATION, OPERATIONAL) with the kinds of evidence that justify each stage. The point is that "works" must always name the evidence behind it.

## What the repo already has

From the workflow file names in `.github/workflows/` (contents not reviewed for this document): `ci.yml`, `rust-unit-tests.yml`, `integration-tests.yml`, `coverage.yml`, `canon-proof-gate.yml`, `canon-integrity.yml`, `canon-runtime.yml`, `governance-compliance.yml`, `agent-hygiene.yml`, `refuse-list.yml`. Unit tests, integration tests and coverage are therefore already in place in some form. What is not known is how much of the system they actually exercise. Measuring that is step 1 below.

## The verification ladder

Each rung is stronger and costlier than the one below. Use the cheapest rung that can actually catch the failure you care about.

| Rung | What it proves | Tool | Stage it supports |
|---|---|---|---|
| 1. Unit tests | One function behaves as written on chosen inputs | `cargo test` | TEST |
| 2. Property tests | A rule holds for thousands of generated inputs, not just chosen ones | `proptest` | TEST |
| 3. Golden / snapshot tests | Output has not changed unexpectedly | `insta` | TEST |
| 4. Integration tests | Components work together through real interfaces | `cargo test` with `tests/` | INTEGRATION |
| 5. End-to-end tests | A user-visible command does the whole job (`gaia start` really starts something) | CLI test harness, run in CI | INTEGRATION |
| 6. Fuzzing | Parsers and gateways survive hostile or malformed input | `cargo-fuzz` | VERIFICATION |
| 7. Simulation | Behaviour over time or under load, with failures injected (dropped messages, slow disks, crashed nodes) | deterministic simulation, `loom` for concurrency | VERIFICATION |
| 8. Formal checks | A critical rule cannot be violated, over all inputs | `kani` for Rust, or a model in TLA+ | VERIFICATION |
| 9. Mutation testing | The tests themselves would notice a bug | `cargo-mutants` | VERIFICATION |
| 10. Benchmarks | Speed and memory claims are measured, not asserted | `criterion` | VERIFICATION |
| 11. Soak and field evidence | It keeps working when actually run for days, by real use | long-running deployment, logs | OPERATIONAL |

Rungs 8 and 9 are for the few modules where a bug is expensive (security, permissions, the refuse list, anything touching data a person trusted to GAIA). They are not for everything.

## What counts as a claim

The stub problem in #1304 happened because a command reported success with no evidence. The rule that prevents it:

- Any status, doc or message that says something works must link to a test, simulation, or run that shows it.
- A claim with no evidence is written as "specified" or "not implemented", never as "done".
- Evidence is recorded where it can be found: the PR body (`## Stage reached`) and, for lasting claims, a short entry in an evidence ledger (proposed: `docs/evidence/`, one file per subsystem listing claim, test or run, date, commit).

## Simulation, concretely

Simulation is the main way to test a system that is meant to run continuously and coordinate many parts, which is what GAIA is. A minimal useful start:

1. Pick one subsystem with clear inputs and outputs (the orchestrator is a candidate).
2. Write a fake clock and a fake network so a test controls time and delivery.
3. Generate a random but seeded sequence of events, including failures.
4. Check invariants after every step (for example, "a refused action is never executed").
5. When an invariant breaks, the seed reproduces the failure exactly.

This is cheap to start, catches the hard bugs that unit tests miss, and its failures are replayable.

## First steps, in order

1. **Measure.** Run `cargo llvm-cov` (or read the existing `coverage.yml` output) and list which crates have no meaningful tests. This needs a Rust toolchain; see `ENVIRONMENT.md`.
2. **Wire the checkers into CI.** Run `tools/agent-skills/tests/test_skills.py`, `dod_check.py` on PR bodies, and `stub_detector.py` on the Rust sources.
3. **Add `proptest` to one crate** with a rule that is easy to state, such as parse then serialise returning the original value.
4. **Add one end-to-end test** for the first command that really works, so "it runs" is checked on every PR.
5. **Build the first simulation** for the orchestrator, with two invariants.
6. **Start the evidence ledger** with the claims that are already true and tested.

## Not yet verified

- Contents of the existing workflows, and what they cover.
- Whether `proptest`, `criterion`, `insta`, `loom`, `kani` or `cargo-fuzz` are already used in the workspace. A code search returned no results, but that search was incomplete and should not be read as "absent".
- Whether the tools above fit in the sandbox, or must run in GitHub Actions.
