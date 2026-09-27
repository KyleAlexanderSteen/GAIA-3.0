# AI engineering roadmaps — #1065

Listed curriculum. Not a career ranking. Not a job promise. Completing a module does **not** grant merge rights.

Reuse: UKD/AIKD/skills corpus, `gaia-spec/knowledge/`, `docs/research/ml-taxonomy-1052.md`, #341, #1059. Vendor names below are examples.

Review date: 2026-09-27.

## Track A — Machine Learning Foundations

- **Prereq:** high-school algebra; a local Python.
- **Outcomes:** load a CSV; compute a split; report a metric; refuse to train a GAIA crate.
- **Sources:** OPTIMIZERS.md; #1053 refuse trainer.
- **Lab:** write a test that asserts `gaia-learn` is absent (REFUSE.md R-LEARN).
- **Rubric:** Pass = metric named + trainer refused. Fail = adds a training crate.
- **Evidence:** PR that only adds a listed note or a refuse test.
- **Safety:** no live weights.
- **Map:** `gaia-spec/knowledge/`, #1052–#1053.
- **Capstone:** one-page "why this repo will not ship GBM."

## Track B — Generative SI engineering

- **Prereq:** Track A outcomes or equivalent reading.
- **Outcomes:** cite a chunk; mark NeedVerify; do not claim calibrated confidence.
- **Sources:** `gaia-aikd` retrieve_and_cite; #1076 conflict fixture.
- **Lab:** run `cargo test -p gaia-aikd conflict`.
- **Rubric:** Pass = conflicting years stay NeedVerify.
- **Evidence:** test log.
- **Safety:** no HF upload.
- **Map:** `gaia-aikd/`, catalog.json `runtime_enabled=false`.
- **Capstone:** grounded answer with citations or NeedVerify.

## Track C — Agentic systems

- **Prereq:** Track B + read `gaia-acp` README.
- **Outcomes:** name authorize → audit → halt; explain why FakeAdapter still executes.
- **Sources:** #341, #1059, MCP 2026-07-28 profile.
- **Lab:** `cargo test -p gaia-acp actor_cannot`.
- **Rubric:** Pass = actor cannot self-grade or self-resume.
- **Evidence:** test log + one-sentence limit ("no child MCP process until #1061").
- **Safety:** STREAMABLE_HTTP_ENABLED stays false.
- **Map:** `gaia-acp/`, #1042 #1045 #1061.
- **Capstone:** threat note for token passthrough (do not enable HTTP).

## Track D — Responsible SI

- **Prereq:** none. This track is **not** last.
- **Outcomes:** fill one listed inventory row; name a human halt path.
- **Sources:** `docs/governance/`, CARE notes, REFUSE.md.
- **Lab:** add nothing to production endpoints.
- **Rubric:** Pass = template filled locally. Fail = claims an EU filing.
- **Evidence:** uncommitted copy of a governance markdown.
- **Safety:** no PII in the repo.
- **Map:** #1047–#1051, #1045.
- **Capstone:** who can clear a kill (human: only).

Responsible SI constraints apply **inside** A–C labs, not after them.
