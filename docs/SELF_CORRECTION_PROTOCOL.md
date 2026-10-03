# Self-Correction Protocol

Status: documentation only. Operational procedure, not a runtime enforcer.

Anchor: [moral-architecture.md](canon/moral-architecture.md) Principle 7 —
*Correction without shame. Errors are signals. The right response is correction,
not punishment.*

Companion: [`TRACE_BOUNDARY_STRIDE_SRE_SAELA.md`](TRACE_BOUNDARY_STRIDE_SRE_SAELA.md)

---

## When to use this document

Whenever a signal arrives that something in the repository is wrong —
a failing gate, a drifted boundary, an overclaimed feature, or a
knowledge assertion missing its epistemic state — this document
describes the correct response.

The correct response is always the same shape:
1. Name the signal clearly.
2. Identify what it is telling you.
3. Apply the matching procedure.
4. Leave a traceable record.

There is no shame tier. There is no blame section.
The error is a signal. Signals are useful.

---

## Signal taxonomy

Four signal classes cover every correction case in GAIA.

| Class | Trigger | Typical source |
| --- | --- | --- |
| `CI_GATE_FAIL` | A CI check exits non-zero | Proof gate, lint, unit test, schema validation |
| `CANON_DRIFT` | A crate, script, or runtime behaviour exceeds a listed canon boundary | Code review, audit, manual inspection |
| `CLAIM_BREACH` | A doc or commit asserts something not listed, not proven, or not implemented | PR review, canon cross-reference |
| `EPISTEMIC_VIOLATION` | A knowledge listing lacks a confidence level, epistemic state, or falsification pointer | #953 audit, retrieval review |

---

## Correction procedure

### CI_GATE_FAIL

The gate is telling you a specific file is missing a required artefact
or a required property. Trust the gate message exactly — it names the
file and the failure reason.

1. **Read the gate output in full.** Identify every failing file, not
   just the first one.
2. **Determine fix class:**
   - Missing proof ID in a canon doc → add `> Proof: PROOF-XXX` to the
     header block and create `proofs/PROOF-XXX.md`.
   - Proof file missing required section → add `## Method`, `**Type:**`,
     `**Status:**`, `**Method:**` frontmatter.
   - Test failure → fix the code or the test; never delete the test.
   - Schema validation failure → fix the schema or the instance;
     never widen the schema to silence the error.
3. **Identify the correct target branch.** Do not infer the branch name
   from the PR title or issue number. Read the PR metadata directly
   (see the *Wrong-branch class* section below) and verify `head.ref`
   before pushing.
4. **Apply the fix to the correct branch.**
5. **Verify locally when possible.** Run `bash scripts/check_canon_proofs.sh`
   or the relevant check before pushing.
6. **Commit message:** cite the canon constraint. Example:
   `fix(proof-gate): add PROOF-FOO-001 — C77: no canon without proof`

---

### CANON_DRIFT

A crate, script, workflow, or runtime behaviour has grown past the
boundary stated in a canon document. The canon document is the authority.
The drifted artefact must be corrected, not the canon.

1. **Identify the canon document** that defines the boundary.
2. **Quote the exact clause** being violated in your PR description.
3. **Determine rollback scope:**
   - Code exceeds listed boundary → remove the excess; do not promote
     the boundary until a canon amendment is approved.
   - Doc claims a feature not yet implemented → downgrade to listed
     language (`listed; not implemented here`).
   - Runtime enforces a constraint the canon says is advisory only →
     revert enforcement to advisory.
4. **If the boundary itself is wrong,** open a canon amendment via
   [`AMENDMENT_PROTOCOL.md`](canon/AMENDMENT_PROTOCOL.md). Do not
   silently expand the boundary in code.
5. **Commit message:** name the drifted artefact and the canon clause.
   Example: `fix(canon-drift): remove enforcement claim — moral-arch P4,
   non-extraction posture`

---

### CLAIM_BREACH

A document or commit message asserts a capability, integration, or
behavioural property that is not listed in canon, not backed by a proof
artefact, or not yet implemented in the codebase.

1. **Find the exact sentence or bullet** that overclaims.
2. **Classify the overclaim:**
   - Premature capability claim → reword to `planned` / `listed; not
     implemented`.
   - Missing proof backing → create the proof artefact first, then
     restore the claim.
   - Factually wrong → correct it; add a canon reference if one exists.
3. **Check companion documents** for the same overclaim propagated
   elsewhere (README, PR body, issue description).
4. **Apply the minimum correction** that removes the false assertion
   without deleting true information.
5. **Commit message:** name the claim and the correction class.
   Example: `fix(claim-breach): downgrade runtime claim to listed —
   TRACE_BOUNDARY schema-not-integration rule`

---

### EPISTEMIC_VIOLATION

A knowledge listing, retrieval result, or stored chunk is presented
without a confidence level, epistemic state tag, or falsification pointer.
This is the class of error the Citrine Tablet (C210) and the epistemic
state layer (#953) exist to prevent.

1. **Identify the listing or chunk** lacking epistemic metadata.
2. **Apply the correct state tag** from the C210 / #953 taxonomy:
   - `Confirmed` — verified against a primary source with proof artefact
   - `Inferred` — derived with stated confidence; falsification pointer required
   - `Contradicted` — conflicts with a higher-confidence listing; do not suppress
   - `Unresolved` — genuinely unknown; Bridge plane listing
   - `Cannot_Know` — Sacred plane; access requires consent
3. **Add a falsification pointer** for any `Inferred` listing: what
   evidence would change its state to `Confirmed` or `Contradicted`?
4. **Do not delete `Contradicted` listings.** A contradiction is
   information. Suppressing it creates the lie. Expose it with state tag
   and let the retrieval layer surface the conflict.
5. **Commit message:** name the listing and the state assigned.
   Example: `fix(epistemic): tag inference as Inferred + falsification
   pointer — C210 epistemic state layer`

---

## The wrong-branch class

This failure class is common when an agent or tool infers a branch name
from context (PR title, issue number, prior conversation) rather than
reading the actual PR metadata.

**Symptoms:**
- A push succeeds but the CI check on the PR does not re-trigger.
- Files appear on a branch that is not the PR head.
- A second push is required to the correct branch.

**Prevention checklist (run before every push):**

1. Call the PR read tool / `gh pr view` and capture `head.ref` explicitly.
2. Confirm the branch name character-for-character — do not reconstruct
   it from the PR title or issue number.
3. Push to the exact `head.ref` value, not an inferred variant.
4. After pushing, verify the PR's latest commit SHA matches your push
   response before declaring success.

**Recovery:**

1. Identify the correct branch from PR metadata.
2. Re-push all affected files to the correct branch in a single commit.
3. Confirm the CI check re-triggers on the correct PR.
4. The stray branch (wrong target) may be left; it does not need to be
   deleted immediately, but should be cleaned up in the same sprint.

---

## What self-correction is not

- **Not a runtime enforcer.** This document does not introduce a syscall,
  ACP hook, or automated rollback mechanism. Correction is a human and
  agent procedure, not a gate.
- **Not a blame document.** No section records who made the error.
  Commits are traceable; this document does not add a shame layer on top.
- **Not a substitute for AMENDMENT_PROTOCOL.** If the correct action is
  to change what canon says, use
  [`AMENDMENT_PROTOCOL.md`](canon/AMENDMENT_PROTOCOL.md). This document
  covers correcting artefacts to match canon, not changing canon to match
  artefacts.
- **Not exhaustive.** Novel signal classes will emerge. When they do,
  open an issue, add a new section here, and record the first instance as
  a case study in the commit message.

---

## Canon references

- C30 — no silent failures
- C77 — no canon without proof
- C210 — Citrine Tablet / epistemic state layer
- [`moral-architecture.md`](canon/moral-architecture.md) Principle 7
- [`AMENDMENT_PROTOCOL.md`](canon/AMENDMENT_PROTOCOL.md)
- [`TRACE_BOUNDARY_STRIDE_SRE_SAELA.md`](TRACE_BOUNDARY_STRIDE_SRE_SAELA.md)
- [#953](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/953) epistemic state layer
