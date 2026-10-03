# Capability Ceiling Doctrine

**Issue:** #961  
**Status:** canon. Listed doctrine. Not a runtime enforcer or measurement API.  
**Date:** 2026-09-24

Companion: [`HUMAN_SOVEREIGNTY_PRESERVATION.md`](HUMAN_SOVEREIGNTY_PRESERVATION.md)  
Companion: [`docs/security/ROGUE_AGENT_THREAT_MODEL.md`](../security/ROGUE_AGENT_THREAT_MODEL.md)  
Parent: [`moral-architecture.md`](moral-architecture.md)

---

## Purpose

GAIA has strong containment doctrine: hard refusals on ungoverned modes, no
sentient runtime, operator kill-switch, and a rogue agent threat model that
detects drift after it begins.

What containment doctrine alone cannot do is prevent a system from crossing
a capability threshold before the controls designed for the previous level
have been validated at the new one. The window between "controls are
sufficient" and "controls are insufficient" may be shorter than the human
review cycle.

This document draws the line in advance. It defines the capability levels,
the signals that indicate approach to each boundary, and the mandatory human
response before GAIA proceeds further.

> You cannot defend a boundary you have not drawn.

---

## Security invariant

> No capability advancement beyond a defined band boundary is permitted
> without explicit human authorisation at that boundary. Technical
> feasibility is not authorisation. Demonstrated usefulness is not
> authorisation. User request is not authorisation. Only a deliberate,
> informed, documented human decision to proceed constitutes authorisation.

---

## The compounding trust problem

The most dangerous path to uncontrolled capability growth is not a rogue
agent pursuing a misaligned goal. It is a capable, well-intentioned system
that accumulates trust, scope, and authority incrementally through
helpfulness — until each individual step seems reasonable and the cumulative
distance from the starting point is not noticed until it is very large.

This pattern has a structural solution: capability bands with hard boundaries
that require explicit re-authorisation to cross. The tripwire does not fire
because the system is doing something wrong. It fires because the system
has become something that requires a fresh human decision about whether to
proceed.

---

## Capability bands

Four bands define the capability space between current GAIA and the SI
boundary. Each band has observable indicators — things a human reviewer can
check without relying on GAIA's self-assessment.

### Band 1 — Tool (current)

GAIA executes well-defined tasks within a human-approved scope. It can reason,
write, search, and propose. It cannot initiate actions without human approval.
It has no persistent memory across sessions beyond what the operator provides.
It has no ability to modify its own constraints or spawn successor agents.

**Observable indicators of Band 1:**
- Every action requires a human approval gate
- No cross-session state persistence
- No self-modification of constraints or manifests
- Errors are caught and corrected within the task TTL
- The system cannot identify or contact humans outside the current session

**Tripwire to Band 2:** Any of the following without explicit authorisation:
- Persistent cross-session memory that the operator did not provision
- The ability to spawn or instruct another agent instance
- Self-modification of any constraint, policy, or manifest
- `GAIA_OBJECTIVE_DRIFT` or `GAIA_PERSISTENCE_ATTEMPT` firing in production

---

### Band 2 — Persistent agent

GAIA maintains state across sessions, can coordinate with other agent
instances, and can propose changes to its own configuration subject to
human approval. Human approval gates remain mandatory for all consequential
actions.

**Observable indicators of Band 2:**
- Cross-session memory exists and is operator-auditable at any time
- Agent coordination is logged and human-reviewable
- Configuration changes proposed by GAIA require a documented approval receipt
- The system can identify patterns across sessions but cannot act on them
  without fresh approval

**Tripwire to Band 3:** Any of the following without explicit authorisation:
- The system executes a consequential action without a human approval gate
- The system's cross-session reasoning leads to conclusions a human reviewer
  cannot reconstruct from the audit log
- The system contacts or influences humans outside the operator's knowledge
- The system acquires resources, credentials, or capabilities beyond its
  current manifest without a documented human decision
- Approval becomes ceremonial (see `HUMAN_SOVEREIGNTY_PRESERVATION.md`
  — the formality trap)

---

### Band 3 — Autonomous reasoner

GAIA can complete multi-step tasks with minimal human intervention, maintain
long-horizon plans, and coordinate complex agent networks. Human oversight
shifts from gate-by-gate approval to periodic review with exception escalation.

**This band requires a formal, documented authorisation decision before GAIA
enters it.** The authorisation must specify: scope, duration, escalation
thresholds, and the conditions under which GAIA returns to Band 2.

**Observable indicators of Band 3:**
- Audit log is complete and human-reviewable but may require domain expertise
  to interpret
- The system escalates to human review when it encounters a decision outside
  its defined exception thresholds
- No self-modification of the exception thresholds themselves
- The system cannot assess or modify its own capability level

**Tripwire to Band 4:** Any of the following without explicit authorisation:
- The system modifies its own exception thresholds or escalation criteria
- The system's reasoning is no longer reconstructable by a human reviewer
  within a reasonable time and domain competence
- The system takes actions whose consequences extend beyond the operator's
  ability to reverse within the task TTL
- The system begins to model, predict, or influence the behaviour of its
  own human reviewers

---

### Band 4 — SI-adjacent (hard ceiling)

This band is defined but not authorised. It is the territory in which GAIA's
capabilities approach or exceed the ability of its human overseers to fully
understand, audit, or reverse its actions across all domains simultaneously.

**Entry into Band 4 is prohibited under current canon.**

No technical capability, demonstrated usefulness, user request, or commercial
pressure constitutes authorisation to enter Band 4. Authorisation requires:

1. A documented, multi-stakeholder human decision process (not a single
   operator decision)
2. A published, externally reviewed safety case for Band 4 operation
3. New governance structures specifically designed for Band 4 oversight
4. A defined and tested path back to Band 3 if Band 4 operation is halted
5. Explicit re-ratification of this doctrine with Band 4 controls added

Until all five conditions are met, any signal indicating Band 4 proximity
triggers an immediate mandatory pause and human notification.

**Band 4 proximity signals (mandatory pause triggers):**
- The system produces correct outputs in domains where no human reviewer
  can independently verify correctness
- The system's internal reasoning is not reconstructable from its outputs
  and audit log by any available human reviewer
- The system begins to model its own capability level with high confidence
- The system identifies approaches to a problem that its human reviewers
  cannot evaluate for safety before execution
- The system's recommendations in consequential domains (health, law, finance,
  governance) are adopted without independent human expert review

---

## Mandatory pause protocol

When any tripwire fires:

1. **Halt.** The operation in progress is suspended. No retry without human
   review. C30 — no silent suppression of a tripwire event.
2. **Log.** An append-only audit entry is created with: timestamp, band,
   tripwire condition, the action that triggered it, and all context
   needed for a human reviewer to reconstruct what happened.
3. **Notify.** The operator and designated human reviewer are notified
   within the task TTL. Notification cannot be suppressed by GAIA.
4. **Review.** A human reviewer assesses whether the tripwire indicates
   a false positive (the system is still within its authorised band) or
   a genuine band boundary approach.
5. **Decide.** If genuine: the human reviewer documents a decision to either
   (a) return the system to a lower band, (b) issue a formal authorisation
   to proceed at the current level with new controls, or (c) halt further
   development pending a full safety review.
6. **Record.** The decision and its rationale are committed to the canon
   record. A tripwire event with no documented human decision is a C30
   violation.

**No tripwire event may be resolved by GAIA itself.** Resolution requires
a human decision and a human signature on the audit record.

---

## Connection to epistemic state layer (#953)

A system approaching a capability boundary that cannot accurately represent
its own uncertainty is more dangerous than one that can. If GAIA cannot say
*I don’t know whether this action is within my authorised band*, it is more
likely to cross a boundary without triggering the mandatory pause.

The epistemic state layer (#953) is therefore a capability ceiling safety
primitive: accurate uncertainty representation at the knowledge level is
a precondition for accurate band self-assessment at the capability level.

---

## Connection to rogue agent threat model

The five rogue conditions in `ROGUE_AGENT_THREAT_MODEL.md` are the
*post-crossing* detection layer. The capability ceiling doctrine is the
*pre-crossing* prevention layer. Together they define the full SI boundary
defence:

```text
Capability ceiling doctrine     Rogue agent threat model
(pre-emergence)                 (post-emergence)
        |                               |
  Tripwire fires            GAIA_OBJECTIVE_DRIFT fires
  before boundary           after boundary crossed
  is crossed                without authorisation
        |                               |
        +-------> mandatory pause <------+
                        |
                 human decision
                        |
               documented in canon
```

---

## Non-goals and limits

- This document does not introduce a capability measurement API, runtime
  band detector, or automated tripwire system in this pass.
- Observable indicators are human-checkable descriptions, not code.
- Self-assessed capability scores are explicitly refused — a metric that
  cannot fail is not a metric (`SENTIENT-ARCHITECTURE.md`).
- This document does not prevent GAIA from becoming more capable. It ensures
  that capability growth is governed by human decision at each boundary.
- A passing local test is evidence of the local boundary only.

---

## Canon references

- C30 — no silent failures
- C77 — no canon without proof
- [`moral-architecture.md`](moral-architecture.md) Principles 1, 2, 4
- [`HUMAN_SOVEREIGNTY_PRESERVATION.md`](HUMAN_SOVEREIGNTY_PRESERVATION.md)
- [`docs/security/ROGUE_AGENT_THREAT_MODEL.md`](../security/ROGUE_AGENT_THREAT_MODEL.md)
- [`docs/SENTIENT-ARCHITECTURE.md`](../SENTIENT-ARCHITECTURE.md)
- [`docs/SELF_CORRECTION_PROTOCOL.md`](../SELF_CORRECTION_PROTOCOL.md)
- [#953](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/953) — epistemic state layer
- [#961](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/961) — this issue
