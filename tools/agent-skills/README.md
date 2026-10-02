# Agent skill checkers

Small, runnable checks that implement skills from the AI skills registry (`docs/knowledge/UNIVERSAL-AI-SKILLS-DATABASE.md`, realm 4 coding and realm 5 reasoning; `gaia-aikd/src/skills.rs` ids `debugging` and `error-recovery`). Standard library only.

| Tool | Skill | Issue | Exit code |
|---|---|---|---|
| `stub_detector.py [root]` | Finds Rust functions that print a success line but still contain a TODO | #1304 | 1 if any found |
| `dod_check.py [file]` | Checks a PR body has a `## Stage reached` section; `Closes #` needs OPERATIONAL checked | #1319 | 1 on any problem |
| `name_audit.py [root]` | Counts leftover GAIA 2.0 names, ignoring stable ids (`PROOF-GAIA20`, `gaia20_`, `gaia-2-0`) | #1296 | 1 if any leftover |

Tests: `python tools/agent-skills/tests/test_skills.py`.

## Known limits

- `stub_detector.py` only catches stubs that print a success-looking line. A stub that prints "Submitting..." and returns Ok is not flagged.
- `name_audit.py` reads tracked text files only and skips binary files.
- Not wired into CI yet. Wiring is a separate change.
